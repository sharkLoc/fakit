use crate::{
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use regex::RegexBuilder;
use std::path::Path;

pub fn search_fa<P: AsRef<Path> + Copy>(
    file: Option<P>,
    out: Option<P>,
    pat: &str,
    ig: bool,
    header: bool,
    keep: bool,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fp = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();

    info!("regex pattern is: {}", pat);
    let re = RegexBuilder::new(pat)
        .case_insensitive(ig)
        .unicode(true)
        .build()?;
    let mut writer = file_writer(out, compression_level)?;
    if header {
        writer.write_all(b"sequence_name\tstart\tend\tpattern\tlength\tsequence\n")?;
    }

    while rset.fill(&mut fp)? {
        for rec in rset.iter().map_while(Result::ok) {
            let seq = rec.seq_str();
            let result = re.captures_iter(&seq);
            for ret in result {
                let group = ret.len();
                for i in 0..group {
                    if let Some(x) = ret.get(i) {
                        if keep {
                            writer.write_all(rec.id())?;
                        } else {
                            writer.write_all(
                                rec.id_str().split_whitespace().next().unwrap().as_bytes(),
                            )?;
                        }
                        writer
                            .write_all(format!("\t{}\t{}\t", x.start() + 1, x.end()).as_bytes())?;
                        writer.write_all(pat.as_bytes())?;
                        writer.write_all(format!("\t{}\t", x.end() - x.start()).as_bytes())?;
                        writer.write_all(x.as_str().as_bytes())?;
                        writer.write_all(b"\n")?;
                    }
                }
            }
        }
    }
    writer.flush()?;

    Ok(())
}
