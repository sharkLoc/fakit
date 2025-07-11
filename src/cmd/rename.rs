use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

pub fn rename_fa<P: AsRef<Path> + Copy>(
    input: Option<P>,
    keep: bool,
    prefix: Option<String>,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fp = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut writer = file_writer(output, compression_level)?;
    let mut n = 0usize;

    while rset.fill(&mut fp)? {
        for rec in rset.iter().map_while(Result::ok) {
            n += 1;
            if let Some(pre) = &prefix {
                let newid = format!("{}{}", pre, n);
                if keep {
                    let mut id_split = rec.id_str().split_whitespace();
                    id_split.next(); // skip the first part of the ID

                    if id_split.clone().count() > 0 {
                        // If there are multiple parts, write the rest of the ID
                        let desc = id_split.collect::<Vec<&str>>().join(" ");
                        let new = format!("{} {}", newid, desc);
                        write_record(&mut writer, new.as_bytes(), &rec.seq(), line_width)?;
                    } else {
                        write_record(&mut writer, newid.as_bytes(), &rec.seq(), line_width)?;
                    }
                } else {
                    write_record(&mut writer, newid.as_bytes(), &rec.seq(), line_width)?;
                }
            } else {
                if keep {
                    write_record(&mut writer, rec.id(), &rec.seq(), line_width)?;
                } else {
                    let mut id_split = rec.id_str().split_whitespace();
                    if let Some(first_id) = id_split.next() {
                        write_record(&mut writer, first_id.as_bytes(), &rec.seq(), line_width)?;
                    }
                }
            }
        }
    }
    writer.flush()?;

    info!("total renamed records count: {}", n);
    Ok(())
}
