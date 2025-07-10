use crate::{errors::FakitError, utils::file_reader, utils::file_writer};
use log::info;
use paraseq::fasta::{Reader, RecordSet};
use std::collections::HashMap;

pub fn kmer_count(
    input: Option<&String>,
    kmer_len: usize,
    header: bool,
    output: Option<&String>,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut reader = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut writer = file_writer(output, compression_level)?;
    let mut kmers = HashMap::new();
    info!("Kmer counting with kmer length: {}", kmer_len);
    let mut count = 0usize;

    while rset.fill(&mut reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            let seq = rec.seq();
            if seq.len() < kmer_len {
                continue;
            }
            for kmer in seq.windows(kmer_len) {
                *kmers.entry(kmer.to_vec()).or_insert(0_u64) += 1;
            }
        }
    }

    if header {
        writer.write_all(b"kmer\tcount\n")?;
    }
    for (k, v) in kmers {
        writer.write_all(&k)?;
        writer.write_all(format!("\t{}\n", v).as_bytes())?;
        count += 1;
    }
    writer.flush()?;

    info!("total count kmer type: {}", count);
    Ok(())
}
