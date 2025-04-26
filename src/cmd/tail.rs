use crate::errors::FakitError;
use crate::utils::*;
use log::info;
use noodles::fasta::io::{reader::Reader, writer};
use std::io::BufReader;
use std::path::Path;

pub fn tail_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut rdr = file_reader(input).map(BufReader::new).map(Reader::new)?;

    if let Some(file) = input {
        info!("reading from file: {}", file.as_ref().display());
    } else {
        info!("reading from stdin");
    }
    info!("get tail {} records", number);

    let mut total = 0usize;
    for _ in rdr.records() {
        total += 1;
    }
    info!("total fasta sequences number: {}", total);
    let skip_n = total - number;
    let mut wtr = writer::Builder::default()
        .set_line_base_count(line_width)
        .build_from_writer(file_writer(output, compression_level)?);

    let mut rdr2 = file_reader(input).map(BufReader::new).map(Reader::new)?;
    for rec in rdr2.records().skip(skip_n).flatten() {
        wtr.write_record(&rec)?;
    }

    Ok(())
}
