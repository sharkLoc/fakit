use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::fasta::{Reader, RecordSet};
use std::{io::BufReader, path::Path};

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

    let mut count = 0usize;
    'outer: while rset.fill(&mut fdr)? {
        for rec in rset.iter().map_while(Result::ok) {
            if count >= number {
                break 'outer;
            }
            count += 1;
            write_record(&mut wdr, rec.id(), &rec.seq(), line_width)?;
        }
    }
    wdr.flush()?;

    info!("get top {} records", number);
    Ok(())
}
