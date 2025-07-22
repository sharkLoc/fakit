use crate::errors::FakitError;
use log::{error, info};
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

#[derive(Debug, Clone, Copy)]
enum CompressionFormat {
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    Plain,
}

impl CompressionFormat {
    fn magic_number(&self) -> &[u8] {
        match self {
            CompressionFormat::Gzip => &GZ_MAGIC,
            CompressionFormat::Bzip2 => &BZ_MAGIC,
            CompressionFormat::Xz => &XZ_MAGIC,
            CompressionFormat::Zstd => &ZSTD_MAGIC,
            CompressionFormat::Plain => &[],
        }
    }

    fn is_match(&self, buffer: &[u8]) -> bool {
        let magic = self.magic_number();
        buffer.len() >= magic.len() && buffer[..magic.len()] == *magic
    }
}

fn magic_num<P: AsRef<Path> + Copy>(file_name: P) -> Result<[u8; MAGIC_MAX_LEN], FakitError> {
    let mut buffer: [u8; MAGIC_MAX_LEN] = [0; MAGIC_MAX_LEN];
    let mut fp = File::open(file_name)?;
    let _ = fp.read(&mut buffer)?;
    Ok(buffer)
}

fn detect_compression<P: AsRef<Path> + Copy>(
    file_name: P,
) -> Result<CompressionFormat, FakitError> {
    let buffer = magic_num(file_name)?;
    let path = file_name.as_ref();

    if CompressionFormat::Gzip.is_match(&buffer) || path.extension().is_some_and(|ext| ext == "gz")
    {
        Ok(CompressionFormat::Gzip)
    } else if CompressionFormat::Bzip2.is_match(&buffer)
        || path.extension().is_some_and(|ext| ext == "bz2")
    {
        Ok(CompressionFormat::Bzip2)
    } else if CompressionFormat::Xz.is_match(&buffer)
        || path.extension().is_some_and(|ext| ext == "xz")
    {
        Ok(CompressionFormat::Xz)
    } else if CompressionFormat::Zstd.is_match(&buffer)
        || path.extension().is_some_and(|ext| ext == "zst")
    {
        Ok(CompressionFormat::Zstd)
    } else {
        Ok(CompressionFormat::Plain)
    }
}

pub fn file_reader<P>(file_in: Option<P>) -> Result<Box<dyn BufRead + Send>, FakitError>
where
    P: AsRef<Path> + Copy,
{
    if let Some(file_name) = file_in {
        info!("reading from file: {}", file_name.as_ref().display());
        let fp = File::open(file_name)?;

        match detect_compression(file_name)? {
            CompressionFormat::Gzip => Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                flate2::read::MultiGzDecoder::new(fp),
            ))),
            CompressionFormat::Bzip2 => Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                bzip2::read::MultiBzDecoder::new(fp),
            ))),
            CompressionFormat::Xz => Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                xz2::read::XzDecoder::new_multi_decoder(fp),
            ))),
            CompressionFormat::Zstd => Ok(Box::new(BufReader::with_capacity(
                BUFF_SIZE,
                zstd::stream::read::Decoder::new(fp)?,
            ))),
            CompressionFormat::Plain => Ok(Box::new(BufReader::with_capacity(BUFF_SIZE, fp))),
        }
    } else {
        if stdin().is_terminal() {
            error!("{}", FakitError::StdinNotDetected);
            std::process::exit(1);
        }
        info!("reading from stdin");
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}

pub fn file_writer<P>(
    file_out: Option<P>,
    compression_level: u32,
) -> Result<Box<dyn Write + Send>, FakitError>
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
                _ => 3,
            };
            Ok(Box::new(BufWriter::with_capacity(
                BUFF_SIZE,
                zstd::stream::write::Encoder::new(fp, level)?,
            )))
        } else {
            Ok(Box::new(BufWriter::with_capacity(BUFF_SIZE, fp)))
        }
    } else {
        Ok(Box::new(BufWriter::new(io::stdout())))
    }
}
