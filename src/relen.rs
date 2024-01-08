use bio::io::fasta;
use std::io::Result;
use std::time::Instant;
use log::*;
use crate::utils::*;

pub fn relen_fa(
    input: &Option<&str>, 
    length: usize, 
    output: &Option<&str>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}",file);
    } else {
        info!("reading from stdin");
    }

    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    
    if length == 0 {
        for rec in fp.records().flatten() {
            fo.write_record(&rec)?;
        }
        fo.flush()?;
    } else {
        let mut fo = file_writer(output, compression_level)?;
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
