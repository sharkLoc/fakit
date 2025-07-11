use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::{error, info};
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn sort_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    sort_by_name: bool,
    sort_by_seq: bool,
    sort_by_gc: bool,
    sort_by_length: bool,
    reverse: bool,
    out: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fa_reader = file_reader(file).map(Reader::new)?;

    let mut n = 0;
    if sort_by_gc {
        n += 1;
    }
    if sort_by_length {
        n += 1;
    }
    if sort_by_name {
        n += 1;
    }
    if sort_by_seq {
        n += 1;
    }
    if n > 1 {
        error!(
            "only one of the flags -l (--sort-by-length), -n (--sort-by-name), -g (--sort-by-gc) and -s (--sort-by-seq) is allowed"
        );
        std::process::exit(1);
    }
    if n == 0 {
        error!("please specifiy one of the flags: -l, -n, -g, -s");
        std::process::exit(1);
    }

    let mut vec_reads = vec![];

    let mut rset = RecordSet::default();
    while rset.fill(&mut fa_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            vec_reads.push((rec.id_str().to_string(), rec.seq_str().to_string()));
        }
    }
    info!("all records has been readed into memory, start sort ...");
    if reverse {
        info!("output reversed result");
    }

    if sort_by_name {
        info!("sort read by name");
        if reverse {
            vec_reads.sort_by(|a, b| b.0.cmp(&a.0));
        } else {
            vec_reads.sort_by(|a, b| a.0.cmp(&b.0));
        }
    } else if sort_by_seq {
        info!("sort read by sequence");
        if reverse {
            vec_reads.sort_by(|a, b| b.1.cmp(&a.1));
        } else {
            vec_reads.sort_by(|a, b| a.1.cmp(&b.1));
        }
    } else if sort_by_length {
        info!("sort read by length");
        if reverse {
            vec_reads.sort_by_key(|b| std::cmp::Reverse(b.1.len()))
        } else {
            vec_reads.sort_by_key(|a| a.1.len())
        }
    } else if sort_by_gc {
        info!("sort read by gc content");
        if reverse {
            vec_reads.sort_by(|a, b| {
                let r1_gc =
                    a.1.as_bytes()
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / a.1.len() as f64;
                let r2_gc =
                    b.1.as_bytes()
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / b.1.len() as f64;
                r2_gc.partial_cmp(&r1_gc).unwrap()
            });
        } else {
            vec_reads.sort_by(|a, b| {
                let r1_gc =
                    a.1.as_bytes()
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / a.1.len() as f64;
                let r2_gc =
                    b.1.as_bytes()
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / b.1.len() as f64;
                r1_gc.partial_cmp(&r2_gc).unwrap()
            });
        }
    }

    info!("sort done, start to output ...");
    let mut fa_writer = file_writer(out, compression_level)?;
    for rec in vec_reads {
        write_record(
            &mut fa_writer,
            rec.0.as_bytes(),
            rec.1.as_bytes(),
            line_width,
        )?;
    }
    fa_writer.flush()?;

    Ok(())
}
