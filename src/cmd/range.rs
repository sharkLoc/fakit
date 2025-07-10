use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::fasta::{Reader, RecordSet};
use std::path::Path;

pub fn range_fasta<P: AsRef<Path> + Copy>(
    input: Option<P>,
    skip: usize,
    take: usize,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fp_reader = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();
    info!("skip first {} records", skip);
    info!("get {} records", take);

    let mut fp_writer = file_writer(output, compression_level)?;
    let mut skipped = 0usize;
    let mut taken = 0usize;

    'outer: while rset.fill(&mut fp_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            if skipped < skip {
                skipped += 1;
                continue;
            }
            if taken >= take {
                break 'outer;
            }
            if taken < take {
                write_record(&mut fp_writer, rec.id(), &rec.seq(), line_width)?;
                taken += 1;
            }
        }
    }
    fp_writer.flush()?;
    info!("total get sequence number: {}", taken);

    Ok(())
}
