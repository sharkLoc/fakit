use crate::utils::{file_reader, file_writer};
use anyhow::{Error, Result};
use log::info;
use noodles::fasta::io::{reader::Reader, writer};
use std::{io::BufReader, path::Path};

pub fn top_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), Error> {
    let mut rdr = file_reader(input).map(BufReader::new).map(Reader::new)?;

    if let Some(file) = input {
        info!("reading from file: {}", file.as_ref().display());
    } else {
        info!("reading from stdin");
    }
    info!("get top {} records", number);

    let mut wtr = writer::Builder::default()
        .set_line_base_count(line_width)
        .build_from_writer(file_writer(output, compression_level)?);

    for rec in rdr.records().take(number).flatten() {
        wtr.write_record(&rec)?;
    }

    Ok(())
}
