use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::{collections::HashMap, path::Path};

pub fn shuffle_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    seed: u64,
    out: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    info!("rand seed: {}", seed);
    let mut rng = Pcg64::seed_from_u64(seed);

    let mut fa_reader = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut reads_map = HashMap::new();
    let mut index = 0usize;

    while rset.fill(&mut fa_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            reads_map.insert(
                index,
                vec![rec.id().to_owned(), rec.seq_str().as_bytes().to_owned()],
            );
            index += 1;
        }
    }

    info!("all records has been readed into memory, start shuffle ...");
    let mut shuffled_indices: Vec<usize> = (0..index).collect();
    shuffled_indices.shuffle(&mut rng);
    info!("shuffle done, start write to output ...");

    let mut writer = file_writer(out, compression_level)?;
    for idx in shuffled_indices {
        if let Some(reads) = reads_map.get(&idx) {
            write_record(&mut writer, reads[0].as_slice(), &reads[1], line_width)?;
        }
    }
    writer.flush()?;

    Ok(())
}
