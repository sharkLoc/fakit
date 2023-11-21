use std::io::Result;
use std::time::Instant;
use bio::io::fasta;
use rand::{prelude::*, Rng};
use rand_pcg::Pcg64;
use log::*;
use crate::utils::*;


// reduce much memory but cost more time
pub fn select_fasta(
    file: &Option<&str>, 
    n: usize, 
    seed: u64, 
    out: &Option<&str>,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        if let Some(file) = file {
            info!("reading from file: {}",file);
        } else {
            info!("reading from stdin");
        }
        info!("rand seed: {}",seed);
        info!("reduce much memory but cost more time");
    }
    let start = Instant::now();

    let mut rng = Pcg64::seed_from_u64(seed);
    let mut get: Vec<usize> = Vec::with_capacity(n);

    let fa_reader = fasta::Reader::new(file_reader(file)?);
    for (order, _) in fa_reader.records().flatten().enumerate() {
        if order < n {
            get.push(order);
        } else {
            let ret = rng.gen_range(0..=order);
            if ret < n {
                get[ret] = order;
            }
        }
    }

    let fo = file_writer(out)?;
    let mut w = fasta::Writer::new(fo);

    let fa_reader2 = fasta::Reader::new(file_reader(file)?);
    for (order, rec) in fa_reader2.records().flatten().enumerate() {
        if get.contains(&order) {
            w.write(rec.id(), rec.desc(), rec.seq())?;
        }
    }
    w.flush()?;
    
    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }
    Ok(())
}


// fast mode but cost more memory
pub fn select_fasta2(
    file: &Option<&str>, 
    n: usize, 
    seed: u64, 
    out: &Option<&str>,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        if let Some(file) = file {
            info!("reading from file: {}",file);
        } else {
            info!("reading from stdin");
        }
        info!("rand seed: {}",seed);
        info!("fast mode but cost more memory");
    }
    let start = Instant::now();

    let mut rng = Pcg64::seed_from_u64(seed);
    let mut get: Vec<fasta::Record> = Vec::with_capacity(n);

    let fa_reader = fasta::Reader::new(file_reader(file)?);
    for (order, rec) in fa_reader.records().flatten().enumerate() {
        if order < n {
            get.push(rec);
        } else {
            let ret = rng.gen_range(0..=order);
            if ret < n {
                get[ret] = rec;
            }
        }
    }

    let fo = file_writer(out)?;
    let mut w = fasta::Writer::new(fo);
    for rec in get {
        w.write(rec.id(), rec.desc(), rec.seq())?;
    }
    w.flush()?;

    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }
    Ok(())
}