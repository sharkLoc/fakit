use crate::utils::*;
use bio::io::fasta;
use std::io::Result;
use log::*;
use std::time::Instant;
use rand::prelude::*;
use rand_pcg::Pcg64;


pub fn shuffle_fasta(
    file: &Option<&str>,
    seed: u64,
    out: &Option<&str>,
    compression_level: u32, 
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = file {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }
    info!("rand seed: {}",seed);

    let mut rng = Pcg64::seed_from_u64(seed);
    let fa_reader = file_reader(file).map(fasta::Reader::new)?;
    
    let mut vec_reads = vec![];
    for rec in fa_reader.records().flatten() {
        vec_reads.push(rec);
    }
    
    info!("all records has been readed into memory, start shuffle ...");
    vec_reads.shuffle(&mut rng);
    info!("shuffle done, start write to output ...");

    let mut fa_writer = file_writer(out, compression_level).map(fasta::Writer::new)?;
    for rec in vec_reads {
        fa_writer.write_record(&rec)?;
    }
    fa_writer.flush()?;

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}