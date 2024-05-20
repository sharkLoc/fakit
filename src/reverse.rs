use crate::{utils::*, wrap::wrap_fasta};
use anyhow::Result;
use bio::io::fasta;
use log::*;
use std::time::Instant;
use std::{collections::HashMap, path::Path};

pub fn reverse_comp_seq<P: AsRef<Path> + Copy>(
    input: Option<P>,
    out: Option<P>,
    rev: bool,
    line_width: usize,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let maps = HashMap::from([
        (b'A', b'T'),
        (b'T', b'A'),
        (b'G', b'C'),
        (b'C', b'G'),
        (b'N', b'N'),
        (b'a', b't'),
        (b't', b'a'),
        (b'g', b'c'),
        (b'c', b'g'),
        (b'n', b'n'),
    ]);
    let fa_reader = file_reader(input).map(fasta::Reader::new)?;
    let mut out_writer = file_writer(out, compression_level).map(fasta::Writer::new)?;

    for rec in fa_reader.records().flatten() {
        let rev_seq = rec.seq().iter().copied().rev().collect::<Vec<u8>>();

        if rev {
            let seq_new = wrap_fasta(rev_seq.as_slice(), line_width)?;
            out_writer.write(rec.id(), rec.desc(), &seq_new)?;
        } else {
            let rc_seq = rev_seq
                .iter()
                .map(|x| maps.get(x).unwrap_or(&b'N'))
                .collect::<Vec<&u8>>();

            let rev_comp = rc_seq.iter().map(|x| **x).collect::<Vec<u8>>();
            let seq_new = wrap_fasta(rev_comp.as_slice(), line_width)?;

            out_writer.write(rec.id(), rec.desc(), seq_new.as_slice())?;
        }
    }
    out_writer.flush()?;

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
