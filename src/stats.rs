use std::io::{Result};
use bio::io::fasta;
use bio::io::fastq;

use crate::utils::*;

#[derive(Debug)]
struct Fasta {
    seq_num: usize,
    sum_len: usize,
    min_len: usize,
    max_len: usize,
    ave_len: f64,
    count_a: usize,
    count_t: usize,
    count_g: usize,
    count_c: usize,
    count_n: usize,
    rate_gc: f64,
    rate_n: f64,
}

impl Fasta {
    fn new() -> Self {
        Fasta {seq_num: 0, sum_len: 0, min_len: 0, max_len: 0, ave_len: 0.0,
            count_a: 0, count_t: 0, count_g: 0, count_c: 0, count_n: 0, rate_gc: 0.0, rate_n: 0.0,
        }    
    }
    fn ave(&mut self) {
        self.ave_len = self.sum_len as f64 / self.seq_num as f64;
    }
    fn rate(&mut self) {
        let total = self.count_a + self.count_t +self.count_g + self.count_c + self.count_n;
        self.rate_gc = (self.count_g + self.count_c) as f64 / total as f64;
        self.rate_n = self.count_n as f64 / total as f64;
    }
}

pub fn summary_fa(input: &Option<&str>) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(input)?);
    let mut info = Fasta::new();
    for rec in fp.records().flatten() {
        info.seq_num += 1;
        let seq_len = rec.seq().len();
        info.sum_len += seq_len;
        info.min_len = seq_len;
        if seq_len > info.max_len {
            info.max_len = seq_len;    
        }
        if seq_len < info.min_len {
            info.min_len = seq_len;    
        }
    }
    info.ave();
    println!("seq_num: {}\nsum_len: {}\nmin_len: {}\nave_len: {:.2}\nmax_len: {}",
            info.seq_num, info.sum_len, info.min_len, info.ave_len, info.max_len);
    Ok(())
}

pub fn stats_fa(input: &Option<&str>) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(input)?);
    println!("ID\tA\tT\tG\tC\tN\tGC_rate\tN_rate\tSeq_len");
    for rec in fp.records().flatten() {
        let mut info = Fasta::new();
        let seq_len = rec.seq().len();
        for nt in rec.seq().iter() {
            match nt {
                &b'A' | &b'a' => {info.count_a += 1;},
                &b'T' | &b't' => {info.count_t += 1;},
                &b'G' | &b'g' => {info.count_g += 1;},
                &b'C' | &b'c' => {info.count_c += 1;},
                &b'N' | &b'n' => {info.count_n += 1;},
                _ => { 
                    eprintln!("[error]: error base code in seq {}",rec.id());
                    std::process::exit(1);
                }
            }    
        }
        info.rate();
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{}",rec.id(),info.count_a,info.count_t,
        info.count_g,info.count_c,info.count_n,info.rate_gc,info.rate_n,seq_len);
    }
    Ok(())
}


pub fn fake_quality(
    name: &Option<&str>, 
    qual: char,
    out: &Option<&str>,    
) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(name)?);
    let fo = file_writer(out)?;
    let mut w = fastq::Writer::new(fo);
    for rec in fp.records().flatten() {
        let rec_qual = qual.to_string().repeat(rec.seq().len());
        w.write(rec.id(), rec.desc(), rec.seq(), rec_qual.as_bytes())?;
    }
    Ok(())
}
