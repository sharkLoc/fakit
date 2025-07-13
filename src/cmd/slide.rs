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
use std::path::Path;

pub fn sliding_window<P: AsRef<Path> + Copy>(
    step: usize,
    wind: usize,
    file: Option<P>,
    out: Option<P>,
    keep: bool,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fp = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();
    if step == 0 {
        error!("step size can't be 0");
        std::process::exit(1);
    }
    info!("window size : {}", wind);
    info!("step size: {}", step);

    let mut fo = file_writer(out, compression_level)?;
    while rset.fill(&mut fp)? {
        for rec in rset.iter().map_while(Result::ok) {
            let seq = rec.seq();
            let len = seq.len();
            let mut start = 0;
            let mut windows = wind;
            loop {
                if windows < len {
                    let fa = &seq[start..windows];
                    let gc = fa
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / wind as f64;
                    if keep {
                        let id_desc =
                            format!("{} {}-{}:{:.4}", rec.id_str(), start + 1, windows, gc);
                        write_record(&mut fo, id_desc.as_bytes(), fa, line_width)?;
                    } else {
                        fo.write_all(rec.id())?;
                        let desc = format!(" {}-{}:{:.4}\t", start + 1, windows, gc,);
                        fo.write_all(desc.as_bytes())?;
                        fo.write_all(fa)?;
                        fo.write_all(b"\n")?;
                    };
                    start += step;
                    windows += step;
                } else {
                    let fa = &seq[start..len];
                    let gc = fa
                        .iter()
                        .filter(|x| matches!(x, &b'G' | &b'C' | &b'g' | &b'c'))
                        .count() as f64
                        / (len - start) as f64;
                    if keep {
                        let id_desc = format!("{} {}-{}:{:.4}", rec.id_str(), start + 1, len, gc);
                        write_record(&mut fo, id_desc.as_bytes(), fa, line_width)?;
                    } else {
                        fo.write_all(rec.id())?;
                        let desc = format!(" {}-{}:{:.4}\t", start + 1, len, gc,);
                        fo.write_all(desc.as_bytes())?;
                        fo.write_all(fa)?;
                        fo.write_all(b"\n")?;
                    }
                    // rset for next record
                    break;
                }
            }
        }
    }

    fo.flush()?;

    Ok(())
}
