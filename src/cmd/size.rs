use crate::{
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::{info, warn};
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

pub fn size_fasta<P: AsRef<Path> + Copy>(
    input: Option<P>,
    all: bool,
    keep: bool,
    noehader: bool,
    output: Option<P>,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fa_reader = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();

    let mut out = file_writer(output, compression_level)?;
    if all {
        if !noehader {
            out.write_all(b"seq_name\tlength\tcount_A\tcount_T\tcount_G\tcount_C\tcount_N\n")?;
        }
    } else if !noehader {
        out.write_all(b"seq_name\tlength\n")?;
    }
    let mut n = 0usize;

    while rset.fill(&mut fa_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            n += 1;
            let seq = rec.seq();
            if all {
                let mut count_a = 0usize;
                let mut count_t = 0usize;
                let mut count_g = 0usize;
                let mut count_c = 0usize;
                let mut count_n = 0usize;
                for (pos, nt) in seq.iter().enumerate() {
                    match nt {
                        b'A' | b'a' => count_a += 1,
                        b'T' | b't' => count_t += 1,
                        b'G' | b'g' => count_g += 1,
                        b'C' | b'c' => count_c += 1,
                        b'N' | b'n' => count_n += 1,
                        _ => warn!(
                            "Error DNA base code in sequence {} position: {}",
                            rec.id_str(),
                            pos + 1
                        ),
                    }
                }
                let buf = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                    match keep {
                        true => rec.id_str(),
                        false => rec.id_str().split_whitespace().next().unwrap_or(""),
                    },
                    seq.len(),
                    count_a,
                    count_t,
                    count_g,
                    count_c,
                    count_n
                );
                out.write_all(buf.as_bytes())?;
            } else {
                let buf = format!(
                    "{}\t{}\n",
                    if keep {
                        rec.id_str()
                    } else {
                        rec.id_str().split_whitespace().next().unwrap_or("")
                    },
                    seq.len()
                );
                out.write_all(buf.as_bytes())?;
            }
        }
    }
    out.flush()?;
    info!("total sequence number: {}", n);

    Ok(())
}
