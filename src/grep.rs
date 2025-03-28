use crate::utils::*;
use crate::wrap::*;
use anyhow::Error;
use bio::io::fasta;
use log::*;
use regex::RegexBuilder;
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn grep_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    pat: &str,
    case: bool,
    by_id: bool,
    by_seq: bool,
    line_width: usize,
    compression_level: u32,
) -> Result<(), Error> {

    let mut fo = file_writer(out, compression_level).map(fasta::Writer::new)?;

    if let Some(file) = file {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let mut n = 0usize;
    if by_id {
        n += 1;
    }
    if by_seq {
        n += 1;
    }
    match n {
        1 => (),
        0 => {
            error!("please specifiy one of the flags: -n or -s");
            std::process::exit(1);
        }
        _ => {
            error!("only one of the flags -n (--by-name) or -s (--by-seq), is allowed");
            std::process::exit(1);
        }
    }
    info!("regex pattern is: {}", pat);
    let mut n = 0usize;

    let re = RegexBuilder::new(pat)
        .case_insensitive(case)
        .unicode(true)
        .build()?;

    if by_seq {
        let fp = file_reader(file).map(fasta::Reader::new)?;
        for rec in fp.records().flatten() {
            let seq_str = std::str::from_utf8(rec.seq())?;
            if re.captures(seq_str).is_some() {
                n += 1;
                let seq_new = wrap_fasta(rec.seq(), line_width)?;
                fo.write(rec.id(), rec.desc(), seq_new.as_slice())?;
            }
        }
    }
    if by_id {
        let fp = file_reader(file).map(fasta::Reader::new)?;
        for rec in fp.records().flatten() {
            let name = if let Some(desc) = rec.desc() {
                format!("{} {}", rec.id(), desc)
            } else {
                rec.id().to_owned()
            };
            if re.captures(&name).is_some() {
                n += 1;
                let seq_new = wrap_fasta(rec.seq(), line_width)?;
                fo.write(rec.id(), rec.desc(), seq_new.as_slice())?;
            }
        }
    }
    fo.flush()?;

    info!("total match sequences number: {}", n);
    Ok(())
}
