use std::{
    io::Result,
    time::Instant,
};
use bio::io::fasta;
use crate::utils::*;
use log::*;

pub fn split_fa(
    input: &Option<&str>,
    ext: String,
    outdir: Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(input) = input {
        info!("reading form file: {}", input);
    } else {
        info!("reading form stdin");
    }

    let fp = fasta::Reader::new(file_reader(input)?);
    for rec in fp.records().flatten() {
        let path = if let Some(outdir) = outdir { format!("{}/{}.{}",outdir,rec.id(),ext) } else {
            format!("./{}.{}",rec.id(),ext)
        };
        let mut fo = fasta::Writer::new(file_writer(&Some(&path), compression_level)?);
        fo.write_record(&rec)?;
        fo.flush()?;
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}