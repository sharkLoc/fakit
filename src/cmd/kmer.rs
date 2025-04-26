use crate::{errors::FakitError, utils::*};
use bio::io::fasta;
use log::*;
use nthash::nthash;
use std::collections::HashMap;

pub fn kmer_count(
    input: Option<&String>,
    kmer_len: usize,
    header: bool,
    output: Option<&String>,
    compression_level: u32,
) -> Result<(), FakitError> {
    let reader = file_reader(input).map(fasta::Reader::new)?;
    if let Some(file) = input {
        info!("reading from file: {}", file);
    } else {
        info!("reading from stdin");
    }

    let mut writer = file_writer(output, compression_level)?;
    let mut kmers = HashMap::new();

    for rec in reader.records().flatten() {
        let (mut sidx, mut eidx) = (0, kmer_len);
        let khash = nthash(rec.seq(), kmer_len);
        let len = rec.seq().len();

        while eidx <= len {
            let kseq = &rec.seq()[sidx..eidx];
            let khash_this = nthash(kseq, kmer_len)[0];
            if khash.contains(&khash_this) {
                *kmers.entry(kseq.to_owned()).or_insert(0_u64) += 1;
            }
            sidx += 1;
            eidx += 1;
        }
    }

    if header {
        writer.write_all("kmer\tcount\n".as_bytes())?;
    }
    for (k, v) in kmers {
        writer.write_all(k.as_slice())?;
        writer.write_all(format!("\t{}\n", v).as_bytes())?;
    }
    writer.flush()?;

    Ok(())
}
