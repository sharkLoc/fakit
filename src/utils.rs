use crate::errors::FakitError;
use log::error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write, prelude::*, stdin},
    path::Path,
};

const GZ_MAGIC: [u8; 3] = [0x1f, 0x8b, 0x08];
const BZ_MAGIC: [u8; 3] = [0x42, 0x5a, 0x68];
const XZ_MAGIC: [u8; 6] = [0xfd, 0x37, 0x7a, 0x58, 0x5A, 0x00];
const ZSTD_MAGIC: [u8; 4] = [0x28, 0xb5, 0x2f, 0xfd];
const MAGIC_MAX_LEN: usize = 6;
const BUFF_SIZE: usize = 1024 * 1024;

fn magic_num<P: AsRef<Path> + Copy>(file_name: P) -> Result<[u8; MAGIC_MAX_LEN], FakitError> {
    let mut buffer: [u8; MAGIC_MAX_LEN] = [0; MAGIC_MAX_LEN];
    let mut fp = File::open(file_name)?;
    let _ = fp.read(&mut buffer)?;
    Ok(buffer)
}

fn is_gzipped<P: AsRef<Path> + Copy>(file_name: P) -> Result<bool, FakitError> {
    let buffer = magic_num(file_name)?;
    let gz_or_not =
        buffer[0] == GZ_MAGIC[0] && buffer[1] == GZ_MAGIC[1] && buffer[2] == GZ_MAGIC[2];
    Ok(gz_or_not
        || file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "gz"))
}

fn is_bzipped<P: AsRef<Path> + Copy>(file_name: P) -> Result<bool, FakitError> {
    let buffer = magic_num(file_name)?;
    let bz_or_not =
        buffer[0] == BZ_MAGIC[0] && buffer[1] == BZ_MAGIC[1] && buffer[2] == BZ_MAGIC[2];
    Ok(bz_or_not
        || file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "bz2"))
}

fn is_xz<P: AsRef<Path> + Copy>(file_name: P) -> Result<bool, FakitError> {
    let buffer = magic_num(file_name)?;
    let xz_or_not = buffer[0] == XZ_MAGIC[0]
        && buffer[1] == XZ_MAGIC[1]
        && buffer[2] == XZ_MAGIC[2]
        && buffer[3] == XZ_MAGIC[3]
        && buffer[4] == XZ_MAGIC[4]
        && buffer[5] == XZ_MAGIC[5];
    Ok(xz_or_not
        || file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "xz"))
}

fn is_zstd<P: AsRef<Path> + Copy>(file_name: P) -> Result<bool, FakitError> {
    let buffer = magic_num(file_name)?;
    let zstd_or_not = buffer[0] == ZSTD_MAGIC[0]
        && buffer[1] == ZSTD_MAGIC[1]
        && buffer[2] == ZSTD_MAGIC[2]
        && buffer[3] == ZSTD_MAGIC[3];
    Ok(zstd_or_not
        || file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "zst"))
}


pub fn file_reader<P>(file_in: Option<P>) -> Result<Box<dyn BufRead>, FakitError>
where
    P: AsRef<Path> + Copy,
{
    if let Some(file_name) = file_in {
        let fp = File::open(file_name)?;
        let gz_flag = is_gzipped(file_name)?;
        let bz_flag = is_bzipped(file_name)?;
        let zx_flag = is_xz(file_name)?;
        let zstd_flag = is_zstd(file_name)?;

        if gz_flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                flate2::read::MultiGzDecoder::new(fp),
            )))
        } else if bz_flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                bzip2::read::MultiBzDecoder::new(fp),
            )))
        } else if zx_flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                xz2::read::XzDecoder::new_multi_decoder(fp),
            )))
        } else if zstd_flag {
            Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                zstd::stream::read::Decoder::new(fp)?,
            )))
        } else {
            Ok(Box::new(BufReader::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        if stdin().is_terminal() {
            error!("{}", FakitError::StdinNotDetected);
            std::process::exit(1);
        }
        let fp = BufReader::new(io::stdin());
        Ok(Box::new(fp))
    }
}

pub fn file_writer<P>(
    file_out: Option<P>,
    compression_level: u32,
) -> Result<Box<dyn Write>, FakitError>
where
    P: AsRef<Path> + Copy,
{
    if let Some(file_name) = file_out {
        let fp = File::create(file_name)?;
        if file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "gz")
        {
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                flate2::write::GzEncoder::new(fp, flate2::Compression::new(compression_level)),
            )))
        } else if file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "bz2")
        {
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                bzip2::write::BzEncoder::new(fp, bzip2::Compression::new(compression_level)),
            )))
        } else if file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "xz")
        {
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                xz2::write::XzEncoder::new(fp, compression_level),
            )))
        } else if file_name
            .as_ref()
            .extension()
            .is_some_and(|ext| ext == "zst")
        {
            let level = match compression_level {
                1 => 1,
                2 => 3,
                3 => 7,
                4 => 11,
                _ => 3
            };
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                zstd::stream::write::Encoder::new(fp, level)?,
            )))
        } 
        else {
            Ok(Box::new(BufWriter::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        Ok(Box::new(BufWriter::new(io::stdout())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gz_or_not() {
        assert_eq!(is_gzipped("example/uniques.fa.gz").unwrap(), true);
    }

    #[test]
    fn xz_or_not() {
        assert_eq!(is_xz("example/uniques.fa.xz").unwrap(), true);
    }

    #[test]
    fn bzip2_or_not() {
        assert_eq!(is_bzipped("example/uniques.fa.bz2").unwrap(), true);
    }

    #[test]
    fn zstd_or_not() {
        assert!(is_zstd("example/uniques.fa.zst").unwrap());
    }
}
