use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::{info, error};
use paraseq::fasta::{Reader, RecordSet};
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
) -> Result<(), FakitError> {
    let mut fa_reader = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();
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
    } else if bzip2 {
        format!("{}{}.fasta.bz2", out_pre, index)
    } else if xz {
        format!("{}{}.fasta.xz", out_pre, index)
    } else {
        format!("{}{}.fasta", out_pre, index)
    };

    let mut fh = vec![file_writer(Some(&out), compression_level)?];

    info!("start to write file: {}", out);
    while rset.fill(&mut fa_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            if flag < num {
                let mut fhthis = fh.get_mut(index).unwrap();
                write_record(&mut fhthis, rec.id(), &rec.seq(), line_width)?;
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
                fh.push(file_writer(Some(&out), compression_level)?);
                let mut fhthis = fh.get_mut(index).unwrap();

                info!("start to write file: {}", out);
                write_record(&mut fhthis, rec.id(), &rec.seq(), line_width)?;
                flag = 1; // already write one record in this loop, flag add one
            }
        }
    }

    info!("total chunk number is: {}", index + 1);
    Ok(())
}
