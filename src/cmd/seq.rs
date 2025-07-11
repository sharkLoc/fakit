use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::{error, info, trace};
use paraseq::{
    fasta::{Reader, RecordSet},
    fastx::Record,
};
use std::path::Path;

#[allow(clippy::too_many_arguments)]
pub fn seq_fa<P: AsRef<Path> + Copy>(
    input: Option<P>,
    lower: bool,
    upper: bool,
    min_len: Option<usize>,
    max_len: Option<usize>,
    min_gc: Option<f64>,
    max_gc: Option<f64>,
    seq: bool,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut reader = file_reader(input).map(Reader::new)?;

    let mut n = 0;
    if lower {
        n += 1;
        info!("lowercase all bases");
    }
    if upper {
        n += 1;
        info!("uppercase all bases");
    }
    if n > 1 {
        error!("only one of the flags -l (--lower-case), -u (--upper-case) is allowed");
        std::process::exit(1);
    }

    let mut rset = RecordSet::default();
    let mut writer = file_writer(output, compression_level)?;
    let mut count = 0usize;

    while rset.fill(&mut reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            if let Some(min_len) = min_len {
                if rec.seq().len() < min_len {
                    trace!("sequence id: {} skipped, too short", rec.id_str());
                    continue;
                }
            }
            if let Some(max_len) = max_len {
                if rec.seq().len() > max_len {
                    trace!("sequence id: {} skipped, too long", rec.id_str());
                    continue;
                }
            }

            if let Some(min_gc) = min_gc {
                let gc = rec
                    .seq()
                    .iter()
                    .filter(|x| x == &&b'G' || x == &&b'C' || x == &&b'g' || x == &&b'c')
                    .count() as f64
                    / rec.seq().len() as f64;
                if gc < min_gc {
                    trace!(
                        "sequence id: {} skipped, gc content less than required",
                        rec.id_str()
                    );
                    continue;
                }
            }
            if let Some(max_gc) = max_gc {
                let gc = rec
                    .seq()
                    .iter()
                    .filter(|x| x == &&b'G' || x == &&b'C' || x == &&b'g' || x == &&b'c')
                    .count() as f64
                    / rec.seq().len() as f64;
                if gc > max_gc {
                    trace!(
                        "sequence id: {} skipped, gc content more than required",
                        rec.id_str()
                    );
                    continue;
                }
            }

            let seqfa = if lower {
                rec.seq().to_ascii_lowercase()
            } else if upper {
                rec.seq().to_ascii_uppercase()
            } else {
                rec.seq().to_vec()
            };
            count += 1;

            if seq {
                // line_wrap(&seqfa, line_width, &mut writer)?;
                writer.write_all(&seqfa)?;
                writer.write_all(b"\n")?;
            } else {
                write_record(&mut writer, rec.id(), seqfa.as_slice(), line_width)?;
            }
        }
    }
    writer.flush()?;

    info!("total {} sequences output", count);
    Ok(())
}
