use bio::io::fasta;
use std::io::Result;

use crate::utils::*;

pub fn format_fa(
    input: &Option<&str>, 
    length: u64, 
    output: &Option<&str>
) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output)?);
    
    if length == 0 {
        for rec in fp.records().flatten() {
            fo.write_record(&rec)?;
        }
    } else {
        let mut fo = file_writer(output)?;
        for rec in fp.records().flatten() {
            let mut n = 0;
            writeln!(&mut fo, ">{}", rec.id())?;
            let seq_len = rec.seq().len() as u64;

            for nt in rec.seq() {
                n += 1;
                write!(&mut fo, "{}", *nt as char)?;
                if n == length {
                    write!(&mut fo, "{}", "\n")?;
                    n = 0;
                }
            }
            if seq_len % length != 0 {
                write!(&mut fo, "{}", "\n")?;
            }
        }
    }
    Ok(())
}
