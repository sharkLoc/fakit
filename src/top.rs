use crate::utils::*;
use crate::wrap::wrap_fasta;
use anyhow::{Error, Result};
use bio::io::fasta;
use log::*;
use std::{path::Path, time::Instant};

pub fn top_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), Error> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    info!("get top {} records", number);

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    if line_width == 0 {
        for rec in fp.records().take(number).flatten() {
            fo.write_record(&rec)?;
        }
    } else {
        for rec in fp.records().take(number).flatten() {
            let seq_len = rec.seq().len();
            if seq_len <= line_width {
                fo.write_record(&rec)?;
            } else {
                /*let mut seq_new: Vec<&[u8]> = vec![];
                let mut index = 0usize;
                loop {
                    index += line_width;
                    let start = index - line_width;
                    if index <= seq_len {
                        let window = &rec.seq()[start..index];
                        seq_new.push(window);
                        seq_new.push("\n".as_bytes());
                    } else {
                        index = seq_len;
                        let window = &rec.seq()[start..index];
                        seq_new.push(window);
                    }
                    if index == seq_len { break; }
                }
                let seq_wrap = seq_new.concat();
                fo.write(rec.id(), rec.desc(), seq_wrap.as_slice())?;
                */
                let ret = wrap_fasta(rec.seq(), line_width)?;
                fo.write(rec.id(), rec.desc(), ret.as_slice())?;
            }
        }
    }
    fo.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
