use bio::io::fasta;
use std::io::Result;
use std::time::Instant;
use log::*;
use crate::utils::*;

pub fn relen_fa(
    input: &Option<&str>, 
    length: usize, 
    output: &Option<&str>
) -> Result<()> {
    info!("reading from file: {}",input.unwrap());
    let start = Instant::now();

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
            if let Some(desc) = rec.desc() {
                writeln!(&mut fo, ">{} {}", rec.id(), desc)?;
            } else {
                writeln!(&mut fo, ">{}", rec.id())?;
            }
            let seq_len = rec.seq().len() ;

            for nt in rec.seq() {
                n += 1;
                write!(&mut fo, "{}", *nt as char)?;
                if n == length {
                    write!(&mut fo, "{}", "\n")?;
                    n = 0;
                }
            }
            // If it is not a multiple, a newline character is added at the end
            if seq_len % length != 0 {
                write!(&mut fo, "{}", "\n")?;
            }
        }
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}
