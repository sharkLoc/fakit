use std::io::Result;
use bio::io::fasta;
use crate::utils::*;
use log::*;

pub fn split_fa(
    input: String,
    ext: String,
    outdir: Option<&str>,
) -> Result<()> {
    info!("reading form: {}", input);
    let fp = fasta::Reader::new(file_reader(&Some(&input))?);
    for rec in fp.records().flatten() {
        let path = if let Some(outdir) = outdir { format!("{}/{}.{}",outdir,rec.id(),ext) } else {
            format!("./{}.{}",rec.id(),ext)
        };
        let mut fo = fasta::Writer::new(file_writer(&Some(&path))?);
        fo.write_record(&rec)?;
    }

    Ok(())
}