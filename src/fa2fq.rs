use crate::utils::*;
use anyhow::Result;
use bio::io::{fasta, fastq};
use log::*;
use std::{path::Path, time::Instant};

pub fn fake_quality<P: AsRef<Path> + Copy>(
    name: Option<P>,
    qual: char,
    out: Option<P>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = name {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let fp = fasta::Reader::new(file_reader(name)?);
    let fo = file_writer(out, compression_level)?;
    let mut w = fastq::Writer::new(fo);
    for rec in fp.records().flatten() {
        let rec_qual = qual.to_string().repeat(rec.seq().len());
        w.write(rec.id(), rec.desc(), rec.seq(), rec_qual.as_bytes())?;
    }
    w.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
