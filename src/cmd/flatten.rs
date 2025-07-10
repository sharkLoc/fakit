use crate::{errors::FakitError, utils::file_reader, utils::file_writer};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

pub fn flatten_fa<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    keep: bool,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut reader = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();

    let mut writer = file_writer(out, compression_level)?;
    let mut count = 0usize;

    while rset.fill(&mut reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            count += 1;
            if keep {
                writer.write_all(rec.id())?;
                writer.write_all(b"\t")?;
                writer.write_all(rec.seq().as_ref())?;
                writer.write_all(b"\n")?;
            } else {
                let mut id_split = rec.id_str().split_whitespace();
                if let Some(first_id) = id_split.next() {
                    writer.write_all(first_id.as_bytes())?;
                    writer.write_all(b"\t")?;
                    writer.write_all(rec.seq().as_ref())?;
                    writer.write_all(b"\n")?;
                }
            }
        }
    }
    writer.flush()?;

    info!("strip sequence number: {}", count);
    Ok(())
}
