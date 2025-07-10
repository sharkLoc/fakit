use crate::{errors::FakitError, utils::file_reader, utils::file_writer};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

pub fn flatten_fa<P: AsRef<Path> + Copy>(
    file: Option<P>,
    keep: bool,
    gap: bool,
    len: bool,
    gc: bool,
    sep: char,
    out: Option<P>,
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
                writer.write_all(sep.to_string().as_bytes())?;
            } else {
                let mut id_split = rec.id_str().split_whitespace();
                if let Some(first_id) = id_split.next() {
                    writer.write_all(first_id.as_bytes())?;
                    writer.write_all(sep.to_string().as_bytes())?;
                }
            }

            if gap {
                let gap_count = rec
                    .seq()
                    .iter()
                    .filter(|&&c| c == b'N' || c == b'n')
                    .count();
                writer.write_all(gap_count.to_string().as_bytes())?;
                writer.write_all(sep.to_string().as_bytes())?;
            }
            if len {
                let seq_len = rec.seq().len();
                writer.write_all(seq_len.to_string().as_bytes())?;
                writer.write_all(sep.to_string().as_bytes())?;
            }
            if gc {
                let gc_count = rec
                    .seq()
                    .iter()
                    .filter(|&c| *c == b'G' || *c == b'C' || *c == b'g' || *c == b'c')
                    .count();
                let gc_rate = format!("{:.2}", gc_count as f64 / rec.seq().len() as f64 * 100.0);
                writer.write_all(gc_rate.as_bytes())?;
                writer.write_all(sep.to_string().as_bytes())?;
            }

            // write seqquence part
            writer.write_all(rec.seq().as_ref())?;
            writer.write_all(b"\n")?;
        }
    }
    writer.flush()?;

    info!("strip sequence number: {}", count);
    Ok(())
}
