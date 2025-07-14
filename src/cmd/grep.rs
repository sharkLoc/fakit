use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::{error, info};
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use regex::RegexBuilder;
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn grep_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    pat: &str,
    case: bool,
    by_id: bool,
    by_seq: bool,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut reader = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut writer = file_writer(out, compression_level)?;

    let mut flag = 0usize;
    if by_id {
        flag += 1;
    }
    if by_seq {
        flag += 1;
    }

    if flag == 0 {
        error!("please specify one of the flags: -n (--by-name) or -s (--by-seq)");
        std::process::exit(1);
    } else if flag > 1 {
        error!("only one of the flags -n (--by-name) or -s (--by-seq), is allowed");
        std::process::exit(1);
    }

    info!("regex pattern is: {}", pat);
    let re = RegexBuilder::new(pat)
        .case_insensitive(case)
        .unicode(true)
        .build()?;
    let mut counter = 0usize;
    if by_seq {
        while rset.fill(&mut reader)? {
            for rec in rset.iter().map_while(Result::ok) {
                if re.captures(&rec.seq_str()).is_some() {
                    counter += 1;
                    write_record(&mut writer, rec.id(), &rec.seq(), line_width)?;
                }
            }
        }
    }
    if by_id {
        while rset.fill(&mut reader)? {
            for rec in rset.iter().map_while(Result::ok) {
                if re.captures(rec.id_str()).is_some() {
                    counter += 1;
                    write_record(&mut writer, rec.id(), &rec.seq(), line_width)?;
                }
            }
        }
    }
    writer.flush()?;

    info!("total match sequences number: {}", counter);
    Ok(())
}
