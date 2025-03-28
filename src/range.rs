use crate::utils::*;
use crate::wrap::*;
use anyhow::Result;
use bio::io::fasta;
use log::*;
use std::path::Path;

pub fn range_fasta<P: AsRef<Path> + Copy>(
    input: Option<P>,
    skip: usize,
    take: usize,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    
    let fp_reader = file_reader(input).map(fasta::Reader::new)?;

    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    info!("skip first {} records", skip);
    info!("get {} records", take);

    let mut fp_writer = file_writer(output, compression_level).map(fasta::Writer::new)?;
    let mut count = 0usize;
    for rec in fp_reader.records().skip(skip).take(take).flatten() {
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        fp_writer.write(rec.id(), rec.desc(), seq_new.as_slice())?;
        count += 1;
    }
    fp_writer.flush()?;
    info!("total get sequence number: {}", count);
    
    Ok(())
}
