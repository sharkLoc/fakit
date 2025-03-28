use crate::utils::*;
use crate::wrap::*;
use anyhow::Result;
use bio::io::fasta::{self, Record};
use log::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::path::Path;

pub fn shuffle_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    seed: u64,
    out: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    
    let fa_reader = file_reader(file).map(fasta::Reader::new)?;

    if let Some(file) = file {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }
    info!("rand seed: {}", seed);

    let mut rng = Pcg64::seed_from_u64(seed);

    let mut vec_reads = vec![];
    for rec in fa_reader.records().flatten() {
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        let rec_new = Record::with_attrs(rec.id(), rec.desc(), seq_new.as_slice());
        vec_reads.push(rec_new);
    }

    info!("all records has been readed into memory, start shuffle ...");
    vec_reads.shuffle(&mut rng);
    info!("shuffle done, start write to output ...");

    let mut fa_writer = file_writer(out, compression_level).map(fasta::Writer::new)?;
    for rec in vec_reads {
        fa_writer.write_record(&rec)?;
    }
    fa_writer.flush()?;

    Ok(())
}
