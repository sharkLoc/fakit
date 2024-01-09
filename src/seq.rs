use crate::utils::*;
use bio::io::fasta;
use log::*;
use std::{
    io::Result,
    time::Instant,
};


pub fn seq_fa(
    input: &Option<&str>, 
    lower: bool,
    output: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}",file);
    } else {
        info!("reading from stdin");
    }
    if lower {
        info!("lowercase all bases");
    } else {
        info!("uppercase all bases");
    }

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    
    for rec in fp.records().flatten() {
        let seq = if lower { 
            rec.seq().to_ascii_lowercase()
        } else {
            rec.seq().to_ascii_uppercase()
        };
        fo.write(rec.id(), rec.desc(), seq.as_slice())?;
    }
    fo.flush()?;

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}