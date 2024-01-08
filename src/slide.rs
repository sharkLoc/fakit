use crate::utils::*;
use anyhow::Error;
use bio::io::fasta;
use std::time::Instant;
use log::*;


pub fn silding_window(
    step: usize,
    wind: usize,
    file: &Option<&str>,
    out: &Option<&str>,
    keep: bool,
    compression_level: u32,
) -> Result<(), Error> {
    if step == 0 {
        error!("step size can't be 0");
        std::process::exit(1);
    }
    let start = Instant::now();
    if let Some(file) = file {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }
    info!("window size : {}", wind);
    info!("step size: {}", step);

    let fp = fasta::Reader::new(file_reader(file)?);
    let mut fo = file_writer(out, compression_level)?;
    let mut windows = wind;
    for rec in fp.records().flatten() {
        let seq = rec.seq();
        let len = seq.len();
        let mut start = 0;
        loop {
            if windows < len {
                let fa = &seq[start..windows].to_ascii_uppercase();
                let gc = fa.iter().filter(|x| *x == &b'G' || *x == &b'C').count() as f64 / wind as f64;
                let fa_str = std::str::from_utf8(fa)?;
                let out = if keep {
                    format!(">{} {}-{}:{:.4}\n{}\n", rec.id(),start+1, windows, gc, fa_str)
                } else {
                    format!("{}\t{}\t{}\t{:.4}\t{}\n", rec.id(),start+1, windows, gc, fa_str)
                };
                fo.write(out.as_bytes())?;
                start += step;
                windows += step;
            } else {
                let fa = &seq[start..len].to_ascii_uppercase();
                let gc = fa.iter().filter(|x| *x == &b'G' || *x == &b'C').count() as f64 / (len - start) as f64;
                let fa_str = std::str::from_utf8(fa)?;
                let out = if keep {
                    format!(">{} {}-{}:{:.4}\n{}\n",rec.id(), start+1, len, gc, fa_str)
                } else {
                    format!("{}\t{}\t{}\t{:.4}\t{}\n",rec.id(), start+1, len, gc, fa_str)
                };
                fo.write(out.as_bytes())?;
                windows = wind;
                break;
            }
        }
    } 
    fo.flush()?;  
    
    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}
