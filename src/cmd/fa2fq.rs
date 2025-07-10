use crate::{
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::{io::BufReader, path::Path};

pub fn fake_quality<P: AsRef<Path> + Copy>(
    input: Option<P>,
    qual: char,
    keep: bool,
    out: Option<P>,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut rdr = file_reader(input).map(BufReader::new).map(Reader::new)?;
    let mut rset = RecordSet::default();

    let mut wtr = file_writer(out, compression_level)?;
    let qualscore = qual;

    while rset.fill(&mut rdr)? {
        for rec in rset.iter().map_while(Result::ok) {
            wtr.write_all(b"@")?;
            if keep {
                wtr.write_all(rec.id())?;
            } else {
                let mut id_split = rec.id_str().split_whitespace();
                if let Some(first_id) = id_split.next() {
                    wtr.write_all(first_id.as_bytes())?;
                }
            }
            wtr.write_all(b"\n")?;
            wtr.write_all(&rec.seq())?;
            wtr.write_all(b"\n")?;
            wtr.write_all(b"+\n")?;
            let qua = qualscore.to_string().repeat(rec.seq().len());
            wtr.write_all(qua.as_bytes())?;
            wtr.write_all(b"\n")?;
        }
    }
    wtr.flush()?;
    info!(
        "FA2FQ: fake quality scores added with character '{}'",
        qualscore
    );
    Ok(())
}
