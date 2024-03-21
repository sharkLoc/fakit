use std::time::Instant;
use bio::io::{fastq,fasta};
use anyhow::Result;
use crate::utils::*;
use log::*;

pub fn fake_quality(
    name: &Option<&str>, 
    qual: char,
    out: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = name {
        info!("reading from file: {}",file);
    } else {
        info!("reading from stdin");
    }
    
    let fp = fasta::Reader::new(file_reader(name)?);
    let fo = file_writer(out,compression_level)?;
    let mut w = fastq::Writer::new(fo);
    for rec in fp.records().flatten() {
        let rec_qual = qual.to_string().repeat(rec.seq().len());
        w.write(rec.id(), rec.desc(), rec.seq(), rec_qual.as_bytes())?;
    }
    w.flush()?;

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}
