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
use std::{io::BufReader, path::Path};

pub fn tail_n_records<P: AsRef<Path> + Copy>(
    number: usize,
    input: Option<P>,
    two_pass: bool,
    output: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fdr = file_reader(input).map(BufReader::new).map(Reader::new)?;
    let mut rset = RecordSet::default();

    let mut wdr = file_writer(output, compression_level)?;
    if two_pass {
        info!("2-pass mode enabled, reading file twice to get tail records");
        if input.is_none() {
            error!(
                "{}: 2-pass mode is not allowed when reading from stdin. Please provide a file input.",
                FakitError::TwoPassNotAllowedStdin
            );
            std::process::exit(1);
        }

        let mut total = 0usize;
        while rset.fill(&mut fdr)? {
            for _ in rset.iter().map_while(Result::ok) {
                total += 1;
            }
        }
        info!("total fasta sequences number: {}", total);

        let mut fdr2 = file_reader(input).map(BufReader::new).map(Reader::new)?;
        let mut rset2 = RecordSet::default();

        let mut count = 0usize;
        let skip_n = if number >= total {
            total
        } else {
            total - number
        };

        while rset2.fill(&mut fdr2)? {
            for rec in rset2.iter().map_while(Result::ok) {
                if count >= skip_n {
                    write_record(&mut wdr, rec.id(), &rec.seq(), line_width)?;
                }
                count += 1;
            }
        }
    } else {
        let mut records = Vec::with_capacity(number);
        let mut count = 0usize;

        while rset.fill(&mut fdr)? {
            for rec in rset.iter().map_while(Result::ok) {
                let info = (rec.id_str().to_string(), rec.seq_str().to_string());
                if count < number {
                    records.push(info);
                } else {
                    records.remove(0);
                    records.push(info);
                }
                count += 1;
            }
        }
        for (id, seq) in records {
            write_record(&mut wdr, id.as_bytes(), seq.as_bytes(), line_width)?;
        }
    }
    wdr.flush()?;

    info!("get tail {} records", number);
    Ok(())
}
