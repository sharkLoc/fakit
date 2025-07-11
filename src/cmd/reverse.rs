use crate::{
    cmd::wrap::write_record,
    errors::FakitError,
    utils::{file_reader, file_writer},
};
use log::info;
use paraseq::fasta::{Reader, RecordSet};
use std::path::Path;

pub fn reverse_comp_seq<P: AsRef<Path> + Copy>(
    input: Option<P>,
    out: Option<P>,
    rev: bool,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fa_reader = file_reader(input).map(Reader::new)?;
    let mut rset = RecordSet::default();
    let mut out_writer = file_writer(out, compression_level)?;
    let mut conter = 0usize;

    while rset.fill(&mut fa_reader)? {
        for rec in rset.iter().map_while(Result::ok) {
            let seq = rec.seq();
            let new_seq = if rev {
                seq.iter().copied().rev().collect::<Vec<u8>>()
            } else {
                seq.iter()
                    .rev()
                    .map(|x| match x {
                        b'A' => b'T',
                        b'T' => b'A',
                        b'G' => b'C',
                        b'C' => b'G',
                        b'N' => b'N',
                        b'a' => b't',
                        b't' => b'a',
                        b'g' => b'c',
                        b'c' => b'g',
                        b'n' => b'n',
                        _ => b'N',
                    })
                    .collect::<Vec<u8>>()
            };
            write_record(&mut out_writer, rec.id(), &new_seq, line_width)?;
            conter += 1;
        }
    }
    out_writer.flush()?;

    info!("total sequences processed count: {}", conter);
    Ok(())
}
