use std::io::Result;
use crate::utils::*;
use bio::io::fasta;
use std::time::Instant;
use log::*;

pub fn range_fasta(
    input: &Option<&str>,
    skip: usize,
    take: usize,
    output: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }
    info!("skip first {} records", skip);
    info!("get {} records", take);

    let fp_reader = file_reader(input).map(fasta::Reader::new)?;
    let mut fp_writer = file_writer(output, compression_level).map(fasta::Writer::new)?;
    for rec in fp_reader.records().skip(skip).take(take).flatten() {
        fp_writer.write_record(&rec)?;
    }
    fp_writer.flush()?;

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}