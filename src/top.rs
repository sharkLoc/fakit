use crate::utils::*;
use bio::io::fasta;
use std::{
    io::Result,
    time::Instant
};
use log::*;

pub fn top_n_records(
    number: usize,
    input: &Option<&str>,
    output: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}",file);
    } else {
        info!("reading from stdin");
    }
    info!("get top {} records",number);

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    for rec in fp.records().take(number).flatten() {
        fo.write_record(&rec)?;
    }
    fo.flush()?;
    
    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}