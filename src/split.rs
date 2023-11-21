use std::io::Result;
use bio::io::fasta;
use crate::utils::*;
use std::time::Instant;
use log::*;

pub fn split_fa(
    input: &Option<&str>,
    ext: String,
    outdir: Option<&str>,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        if let Some(input) = input {
            info!("reading form file: {}", input);
        } else {
            info!("reading form stdin");
        }
    }
    let start = Instant::now();
    
    let fp = fasta::Reader::new(file_reader(input)?);
    for rec in fp.records().flatten() {
        let path = if let Some(outdir) = outdir { format!("{}/{}.{}",outdir,rec.id(),ext) } else {
            format!("./{}.{}",rec.id(),ext)
        };
        let mut fo = fasta::Writer::new(file_writer(&Some(&path))?);
        fo.write_record(&rec)?;
        fo.flush()?;
    }

    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }
    Ok(())
}