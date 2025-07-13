use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;
use std::path::PathBuf;

pub fn split_fa<P: AsRef<Path> + Copy>(
    input: Option<P>,
    ext: String,
    keep: bool,
    outdir: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fp = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();

    while rset.fill(&mut fp)? {
        for rec in rset.iter().map_while(Result::ok) {
            let id = rec.id_str().split_whitespace().next().unwrap();
            let path = match outdir {
                Some(dir) => dir.as_ref().join(format!("{}.{}", id, ext)),
                None => PathBuf::from(format!("./{}.{}", id, ext)),
            };

            let mut writer = file_writer(Some(&path), compression_level)?;
            match keep {
                true => write_record(&mut writer, rec.id(), &rec.seq(), line_width)?,
                false => write_record(&mut writer, id.as_bytes(), &rec.seq(), line_width)?,
            }
            writer.flush()?;
        }
    }

    Ok(())
}
