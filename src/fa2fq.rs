use crate::utils::{file_reader, file_writer};
use anyhow::Result;
use log::info;
use noodles::fasta::io::reader::Reader;
use noodles::fastq::{io::Writer, record::Definition, Record};
use std::{io::BufReader, path::Path, time::Instant};

pub fn fake_quality<P: AsRef<Path> + Copy>(
    input: Option<P>,
    qual: char,
    out: Option<P>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();
    let mut rdr = file_reader(input).map(BufReader::new).map(Reader::new)?;

    if let Some(file) = input {
        info!("reading from file: {:?}", file.as_ref());
    } else {
        info!("reading from stdin");
    }

    let mut wtr = file_writer(out, compression_level).map(Writer::new)?;
    let qualscore = qual;
    for rec in rdr.records().flatten() {
        let define = if let Some(desc) = rec.description() {
            Definition::new(rec.name(), desc)
        } else {
            Definition::new(rec.name(), "")
        };

        let qua = &qualscore.to_string().repeat(rec.sequence().len());
        let rec_new = Record::new(define, rec.sequence().as_ref(), qua.as_bytes());
        wtr.write_record(&rec_new)?;
    }

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
