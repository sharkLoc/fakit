use crate::utils::*;
use crate::wrap::*;
use anyhow::Result;
use bio::io::fasta::{self, Record};
use log::*;
use std::path::Path;
use std::path::PathBuf;

pub fn split_fa<P: AsRef<Path> + Copy>(
    input: Option<P>,
    ext: String,
    outdir: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(input)?);

    if let Some(input) = input {
        info!("reading form file: {:?}", input.as_ref());
    } else {
        info!("reading form stdin");
    }

    for rec in fp.records().flatten() {
        let path = if let Some(outdir) = &outdir {
            outdir.as_ref().join(format!("{}.{}", rec.id(), ext))
        } else {
            PathBuf::from(format!("./{}.{}", rec.id(), ext))
        };

        let seq_new = wrap_fasta(rec.seq(), line_width)?;
        let rec_new = Record::with_attrs(rec.id(), rec.desc(), seq_new.as_slice());

        let mut fo = fasta::Writer::new(file_writer(Some(&path), compression_level)?);
        fo.write_record(&rec_new)?;
        fo.flush()?;
    }

    Ok(())
}
