use crate::utils::*;
use anyhow::Result;
use bio::io::fasta;
use log::*;
use std::{path::Path, time::Instant};

pub fn flatten_fa<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    keep: bool,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = file {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let reader = file_reader(file).map(fasta::Reader::new)?;
    let mut writer = file_writer(out, compression_level)?;
    let mut count = 0usize;
    for rec in reader.records().flatten() {
        if let Some(desc) = rec.desc() {
            if keep {
                let id = format!("{} {}", rec.id(), desc);
                writer.write_all(id.as_bytes())?;
                writer.write_all("\t".as_bytes())?;
                writer.write_all(rec.seq())?;
                writer.write_all("\n".as_bytes())?;
            } else {
                writer.write_all(rec.id().as_bytes())?;
                writer.write_all("\t".as_bytes())?;
                writer.write_all(rec.seq())?;
                writer.write_all("\n".as_bytes())?;
            }
        } else {
            writer.write_all(rec.id().as_bytes())?;
            writer.write_all("\t".as_bytes())?;
            writer.write_all(rec.seq())?;
            writer.write_all("\n".as_bytes())?;
        }
        count += 1;
    }
    writer.flush()?;

    info!("strip sequence number: {}", count);
    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
