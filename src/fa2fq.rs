use std::io::Result;
use std::time::Instant;
use bio::io::fasta;
use bio::io::fastq;
use crate::utils::*;
use log::*;

pub fn fake_quality(
    name: &Option<&str>, 
    qual: char,
    out: &Option<&str>,
    quiet: bool,    
) -> Result<()> {
    if !quiet {
        if let Some(file) = name {
            info!("reading from file: {}",file);
        } else {
            info!("reading from stdin");
        }
    }
    let start = Instant::now();
    
    let fp = fasta::Reader::new(file_reader(name)?);
    let fo = file_writer(out)?;
    let mut w = fastq::Writer::new(fo);
    for rec in fp.records().flatten() {
        let rec_qual = qual.to_string().repeat(rec.seq().len());
        w.write(rec.id(), rec.desc(), rec.seq(), rec_qual.as_bytes())?;
    }
    w.flush()?;

    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }

    Ok(())
}
