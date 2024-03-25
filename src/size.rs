use anyhow::Error;
use bio::io::fasta;
use crate::utils::*;
use log::*;
use std::time::Instant;

struct Seqinfo {
    count_a: usize,
    count_t: usize,
    count_g: usize,
    count_c: usize,
    count_n: usize,
}

impl Seqinfo {
    fn new() -> Seqinfo {
        Seqinfo { count_a: 0, count_t: 0, count_g: 0, count_c: 0, count_n: 0 }
    }
}

pub fn size_fasta(
    input: &Option<&str>,
    all: bool,
    output: &Option<&str>,
    compression_level: u32,
) -> Result<(), Error> {

    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }

    let fa_reader = file_reader(input).map(fasta::Reader::new)?;
    let mut out = file_writer(output, compression_level)?;
    if all {
        out.write(b"seq_name\tlength\tcount_A\tcount_T\tcount_G\tcount_C\tcount_N\n")?;
    } else {
        out.write(b"seq_name\tlength\n")?;
    }
    let mut recs = fa_reader.records();
    let mut n = 0usize;
    while let Some(each) = recs.next() {
        n += 1;
        let rec = each?;
        if all {
            let mut info = Seqinfo::new();
            let mut pos = 0usize;

            for nt in rec.seq().iter() {
                pos += 1;
                match nt {
                    &b'A' | &b'a' => {info.count_a += 1;},
                    &b'T' | &b't' => {info.count_t += 1;},
                    &b'G' | &b'g' => {info.count_g += 1;},
                    &b'C' | &b'c' => {info.count_c += 1;},
                    &b'N' | &b'n' => {info.count_n += 1;},
                    _ => { 
                        warn!("Error DNA base code in sequence {} position: {}",rec.id(), pos);
                        continue;
                    }
                }    
            }

            let buf = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                rec.id(),
                rec.seq().len(),
                info.count_a,
                info.count_t,
                info.count_g,
                info.count_c,
                info.count_n
            );
            out.write(buf.as_bytes())?;

        } else {
            let buf = format!("{}\t{}\n",rec.id(),rec.seq().len());
            out.write(buf.as_bytes())?;
        }
    }
    out.flush()?;
    info!("total sequence number: {}",n);
    info!("time elapsed is: {:?}", start.elapsed());

    Ok(())
}