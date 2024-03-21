use anyhow::Result;
use crate::utils::*;
use crate::wrap::*;
use bio::io::fasta;
use std::time::Instant;
use log::*;


pub fn range_fasta(
    input: &Option<&str>,
    skip: usize,
    take: usize,
    output: &Option<&str>,
    line_width: usize,
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
    let mut count = 0usize;
    for rec in fp_reader.records().skip(skip).take(take).flatten() {
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        fp_writer.write(rec.id(), rec.desc(), seq_new.as_slice())?;
        count += 1;
    }
    fp_writer.flush()?;
    info!("total get sequence number: {}",count);
    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}