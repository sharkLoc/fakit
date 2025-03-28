use crate::utils::*;
use crate::wrap::*;
use anyhow::{Error, Ok};
use bio::io::fasta;
use log::*;
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn split_chunk<P: AsRef<Path> + Copy>(
    file: Option<P>,
    num: usize,
    gzip: bool,
    bzip2: bool,
    xz: bool,
    out_pre: &str,
    line_width: usize,
    compression_level: u32,
) -> Result<(), Error> {
    let fa_reader = fasta::Reader::new(file_reader(file)?);

    if let Some(file) = file {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    let mut n = 0;
    if gzip {
        n += 1;
    }
    if bzip2 {
        n += 1;
    }
    if xz {
        n += 1;
    }
    if n > 1 {
        error!("only one of the flags --gzip , --xz and --bzip2 is allowed");
        std::process::exit(1);
    }

    let (mut flag, mut index) = (0usize, 0usize);
    let out = if gzip {
        format!("{}{}.fasta.gz", out_pre, index)
        //PathBuf::from("./").join(format!("{}{}.fasta.gz", out_pre, index))
    } else if bzip2 {
        format!("{}{}.fasta.bz2", out_pre, index)
        //PathBuf::from("./").join(format!("{}{}.fasta.bz2", out_pre, index))
    } else if xz {
        format!("{}{}.fasta.xz", out_pre, index)
        //PathBuf::from("./").join(format!("{}{}.fasta.xz", out_pre, index))
    } else {
        format!("{}{}.fasta", out_pre, index)
        //PathBuf::from("./").join(format!("{}{}.fasta", out_pre, index))
    };

    let mut fh = vec![fasta::Writer::new(file_writer(
        Some(&out),
        compression_level,
    )?)];

    info!("start to write file: {}", out);
    for rec in fa_reader.records().flatten() {
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        if flag < num {
            let fhthis = fh.get_mut(index).unwrap();
            fhthis.write(rec.id(), rec.desc(), seq_new.as_slice())?;
            flag += 1;
        } else {
            index += 1;
            let out = if gzip {
                format!("{}{}.fasta.gz", out_pre, index)
            } else if bzip2 {
                format!("{}{}.fasta.bz2", out_pre, index)
            } else if xz {
                format!("{}{}.fasta.xz", out_pre, index)
            } else {
                format!("{}{}.fasta", out_pre, index)
            };
            fh.push(fasta::Writer::new(file_writer(
                Some(&out),
                compression_level,
            )?));
            let fhthis = fh.get_mut(index).unwrap();

            info!("start to write file: {}", out);
            fhthis.write(rec.id(), rec.desc(), seq_new.as_slice())?;
            flag = 1; // already write one record in this loop, flag add one
        }
    }

    info!("total chunk number is: {}", index + 1);
    Ok(())
}
