use crate::utils::*;
use crate::wrap::*;
use anyhow::Result;
use bio::io::fasta;
use log::*;
use std::path::Path;
use std::time::Instant;

pub fn tail_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    info!("get tail {} records", number);
    let start = Instant::now();

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    let mut total = 0usize;

    for _ in fp.records() {
        total += 1;
    }
    info!("total fasta sequences number: {}", total);
    let skip_n = total - number;

    let fp2 = fasta::Reader::new(file_reader(input)?);
    for rec in fp2.records().skip(skip_n).flatten() {
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        fo.write(rec.id(), rec.desc(), seq_new.as_slice())?;
    }
    fo.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
