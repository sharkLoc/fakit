use std::io::Result;
use crate::utils::*;
use bio::io::fasta;
use std::time::Instant;
use log::*;

pub fn top_n_records(
    number: usize,
    input: &Option<&str>,
    output: &Option<&str>,
) -> Result<()> {
    info!("reading from file: {}",input.unwrap());
    info!("get top {} records",number);
    let start = Instant::now();

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output)?);
    let mut counter: usize = 0;
    for rec in fp.records().flatten() {
        if counter < number {
            fo.write_record(&rec)?;
            counter += 1;
        } else {
            break;
        }
    }
  
    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}