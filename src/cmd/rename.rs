use crate::cmd::wrap::*;
use crate::errors::FakitError;
use crate::utils::*;
use bio::io::fasta::{self, Record};
use log::*;
use std::path::Path;

pub fn rename_fa<P: AsRef<Path> + Copy>(
    input: Option<P>,
    keep: bool,
    prefix: Option<String>, //&str,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let fp = fasta::Reader::new(file_reader(input)?);

    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let mut fo = fasta::Writer::new(file_writer(output, compression_level)?);
    let mut n: usize = 0;

    let prefix_fn = |n: usize, rec: &Record| -> String {
        if let Some(pre) = &prefix {
            format!("{}{}", pre, n)
        } else {
            rec.id().to_string()
        }
    };

    for rec in fp.records().flatten() {
        n += 1;
        let newid = prefix_fn(n, &rec);
        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        let record = if keep {
            Record::with_attrs(&newid, rec.desc(), seq_new.as_slice())
        } else {
            Record::with_attrs(&newid, None, seq_new.as_slice())
        };
        fo.write_record(&record)?;
    }
    fo.flush()?;

    Ok(())
}
