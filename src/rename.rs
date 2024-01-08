use bio::io::fasta::{self, Record};
use std::io::Result;
use std::time::Instant;
use log::*;
use crate::utils::*;


pub fn rename_fa(
    input: &Option<&str>,
    keep: bool,
    prefix: Option<String>, //&str,
    output: &Option<&str>,
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
            let record = if keep { 
                Record::with_attrs(&newid, rec.desc(), rec.seq()) 
            } else { 
                Record::with_attrs(&newid, None, rec.seq())
            };
            fo.write_record(&record)?;
        }
        fo.flush()?;
    } else {
        for rec in fp.records().flatten() {
            n += 1;
            let record = if keep { 
                Record::with_attrs(rec.id(), rec.desc(), rec.seq()) 
            } else { 
                Record::with_attrs(rec.id(), None, rec.seq())
            };
            fo.write_record(&record)?;
        }
        fo.flush()?;
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}