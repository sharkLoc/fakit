use bio::io::fasta::{self, Record};
use anyhow::Result;
use std::time::Instant;
use log::*;
use crate::utils::*;
use crate::wrap::*;


pub fn rename_fa(
    input: &Option<&str>,
    keep: bool,
    prefix: Option<String>, //&str,
    output: &Option<&str>,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}",file);
    } else {
        info!("reading from stdin");
    }

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    let mut n: usize = 0;

    if let Some(pre) = prefix {
        for rec in fp.records().flatten() {
            n += 1;
            let newid = format!("{}{}",pre,n);
            let seq_new = wrap_fasta(rec.seq(), line_width)?; 
            let record = if keep {
                Record::with_attrs(&newid, rec.desc(), seq_new.as_slice()) 
            } else { 
                Record::with_attrs(&newid, None, seq_new.as_slice())
            };
            fo.write_record(&record)?;
        }
        fo.flush()?;
    } else {
        for rec in fp.records().flatten() {
            n += 1;
            let seq_new = wrap_fasta(rec.seq(), line_width)?; 
            let record = if keep { 
                Record::with_attrs(rec.id(), rec.desc(), seq_new.as_slice()) 
            } else { 
                Record::with_attrs(rec.id(), None, seq_new.as_slice())
            };
            fo.write_record(&record)?;
        }
        fo.flush()?;
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}