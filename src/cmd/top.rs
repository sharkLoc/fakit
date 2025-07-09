use crate::{
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use std::{io::BufReader, path::Path};
use paraseq::fasta::{Reader, RecordSet};
use crate::cmd::wrap::wrap_fasta2;

pub fn top_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fdr = file_reader(input).map(BufReader::new).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut wdr = file_writer(output, compression_level)?;

    if let Some(file) = input {
        info!("reading from file: {}", file.as_ref().display());
    } else {
        info!("reading from stdin");
    }
    let mut count = 0usize;
    'outer: while rset.fill(&mut fdr)? {
        for rec in rset.iter().map_while(Result::ok) {
            if count >= number {
                break 'outer;
            }
            count += 1;

            wdr.write_all(b">")?;
            wdr.write_all(rec.id())?;
            wdr.write_all(b"\n")?;
            wrap_fasta2(&rec.seq(), line_width, &mut wdr)?;
            wdr.write_all(b"\n")?;
        }
    }

    wdr.flush()?;
    info!("get top {} records", number);
    Ok(())
}
