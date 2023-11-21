use std::io::Result;
use crate::utils::*;
use bio::io::fasta;
use std::time::Instant;
use log::*;

pub fn top_n_records(
    number: usize,
    input: &Option<&str>,
    output: &Option<&str>,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        if let Some(file) = input {
            info!("reading from file: {}",file);
        } else {
            info!("reading from stdin");
        }
        info!("get top {} records",number);
    }
    let start = Instant::now();

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output)?);
    
    for rec in fp.records().take(number).flatten() {
        fo.write_record(&rec)?;
    }
    fo.flush()?;
    
    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }
    
    Ok(())
}