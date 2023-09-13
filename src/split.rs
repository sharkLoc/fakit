use std::io::Result;
use bio::io::fasta;
use crate::utils::*;
use std::time::Instant;
use log::*;

pub fn split_fa(
    input: String,
    ext: String,
    outdir: Option<&str>,
) -> Result<()> {
    info!("reading form file: {}", input);
    let start = Instant::now();
    
    let fp = fasta::Reader::new(file_reader(&Some(&input))?);
    for rec in fp.records().flatten() {
        let path = if let Some(outdir) = outdir { format!("{}/{}.{}",outdir,rec.id(),ext) } else {
            format!("./{}.{}",rec.id(),ext)
        };
        let mut fo = fasta::Writer::new(file_writer(&Some(&path))?);
        fo.write_record(&rec)?;
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}