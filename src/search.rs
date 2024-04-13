use crate::utils::*;
use anyhow::Error;
use bio::io::fasta;
use log::*;
use regex::Regex;
use std::{path::Path, time::Instant};

pub fn search_fa<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    pat: &str,
    header: bool,
    compression_level: u32,
) -> Result<(), Error> {
    let start = Instant::now();
    if let Some(file) = file {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    info!("regex pattern is: {}", pat);

    let re = Regex::new(pat)?;
    let fp = file_reader(file).map(fasta::Reader::new)?;
    let mut fo = file_writer(out, compression_level)?;
    if header {
        fo.write_all("sequence_name\tstart\tend\tpattern\tlength\tsequence\n".as_bytes())?;
    }

    for rec in fp.records().flatten() {
        let seq = rec.seq().to_ascii_uppercase();
        let seq_str = std::str::from_utf8(seq.as_slice())?;
        let result = re.captures_iter(seq_str);
        for ret in result {
            let group = ret.len();
            for i in 0..group {
                if let Some(x) = ret.get(i) {
                    let out_str = format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\n",
                        rec.id(),
                        x.start() + 1,
                        x.end(),
                        pat,
                        x.end() - x.start(),
                        x.as_str()
                    );
                    fo.write_all(out_str.as_bytes())?;
                }
            }
        }
    }
    fo.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
