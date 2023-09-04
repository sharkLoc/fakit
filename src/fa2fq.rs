use std::io::Result;
use bio::io::fasta;
use bio::io::fastq;
use crate::utils::*;

pub fn fake_quality(
    name: &Option<&str>, 
    qual: char,
    out: &Option<&str>,    
) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(name)?);
    let fo = file_writer(out)?;
    let mut w = fastq::Writer::new(fo);
    for rec in fp.records().flatten() {
        let rec_qual = qual.to_string().repeat(rec.seq().len());
        w.write(rec.id(), rec.desc(), rec.seq(), rec_qual.as_bytes())?;
    }
    Ok(())
}