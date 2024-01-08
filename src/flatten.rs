use crate::utils::*;
use bio::io::fasta;
use log::*;
use std::{
    io::Result,
    time::Instant,
};

pub fn flatten_fa(
    file: &Option<&str>, 
    out: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = file {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }

    let reader = file_reader(file).map(fasta::Reader::new)?;
    let mut writer = file_writer(out, compression_level).map(fasta::Writer::new)?;
    let mut count = 0usize;
    for rec in reader.records().flatten() {
        writer.write(rec.id(),rec.desc(),rec.seq())?;
        count += 1;
    }
    writer.flush()?;

    info!("strip sequence number: {}",count);
    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}