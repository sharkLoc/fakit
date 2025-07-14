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
use rand::{Rng, prelude::*};
use rand_pcg::Pcg64;
use std::path::Path;

pub fn select_fasta<P: AsRef<Path> + Copy>(
    file: Option<P>,
    n: usize,
    seed: u64,
    two_pass: bool,
    out: Option<P>,
    line_width: usize,
    compression_level: u32,
) -> Result<(), FakitError> {
    let mut fa_reader = file_reader(file).map(Reader::new)?;
    let mut rset = RecordSet::default();
    info!("rand seed: {}", seed);
    let mut writer = file_writer(out, compression_level)?;

    let mut rng = Pcg64::seed_from_u64(seed);
    let mut order = 0usize;
    if two_pass {
        info!("enable two pass mode");
        let mut get: Vec<usize> = Vec::with_capacity(n);
        while rset.fill(&mut fa_reader)? {
            for _ in rset.iter().map_while(Result::ok) {
                if order < n {
                    get.push(order);
                } else {
                    let ret = rng.random_range(0..=order);
                    if ret < n {
                        get[ret] = order;
                    }
                }
                order += 1;
            }
        }

        order = 0;
        get.sort_unstable(); // keep the order
        info!("all records has been readed into memory, start write to output ...");
        let mut fa_reader2 = file_reader(file).map(Reader::new)?;
        let mut rset2 = RecordSet::default();
        let mut idx = 0usize;
        while rset2.fill(&mut fa_reader2)? {
            for rec in rset2.iter().map_while(Result::ok) {
                if idx < get.len() && order == get[idx] {
                    write_record(&mut writer, rec.id(), &rec.seq(), line_width)?;
                    idx += 1;
                }
                if idx >= get.len() {
                    break;
                }
                order += 1;
            }
        }
    } else {
        let mut get = Vec::with_capacity(n);
        while rset.fill(&mut fa_reader)? {
            for rec in rset.iter().map_while(Result::ok) {
                if order < n {
                    get.push((order, rec.id_str().to_owned(), rec.seq_str().into_owned()));
                } else {
                    let ret = rng.random_range(0..=order);
                    if ret < n {
                        get[ret] = (order, rec.id_str().to_owned(), rec.seq_str().into_owned());
                    }
                }
                order += 1;
            }
        }

        info!("all records has been readed into memory, start write to output ...");
        get.sort_unstable_by_key(|x| x.0); // sort by order to keep the raw order
        for (_, id, seq) in get {
            write_record(&mut writer, id.as_bytes(), seq.as_bytes(), line_width)?;
        }
    }
    writer.flush()?;

    Ok(())
}
