use crate::{
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use noodles::fasta::io::{reader::Reader, writer};
use std::{io::BufReader, path::Path};

pub fn top_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fdr = file_reader(input).map(BufReader::new).map(Reader::new)?;

    if let Some(file) = input {
        info!("reading from file: {}", file.as_ref().display());
    } else {
        info!("reading from stdin");
    }

    let mut fdw = writer::Builder::default()
        .set_line_base_count(line_width)
        .build_from_writer(file_writer(output, compression_level)?);

    for rec in fdr.records().take(number).flatten() {
        fdw.write_record(&rec)?;
    }

    info!("get top {} records", number);
    Ok(())
}
