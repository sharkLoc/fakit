use anyhow::Error;
use bio::io::fasta;
use crate::utils::*;
use log::*;
use std::time::Instant;

#[derive(Debug)]
struct Seqinfo {
    name: String,
    sum_len: usize,
    max_len: usize,
    min_len: usize,
    num_seq: usize,
    count_a: usize,
    count_t: usize,
    count_g: usize,
    count_c: usize,
    count_n: usize,
    rate_gc: f64,
    rate_n: f64,
    mean_len: f64,
}

impl Seqinfo {
    fn new(id: String) -> Self {
        Seqinfo { name: id, sum_len: 0, max_len: 0, min_len: 0, num_seq: 0, count_a: 0, count_t: 0, count_g: 0, count_c: 0, count_n: 0, rate_gc: 0., rate_n: 0., mean_len: 0. }
    }
    fn mean(&mut self) {
        self.mean_len = self.sum_len as f64 / self.num_seq as f64;
    }
    fn rate(&mut self) {
        let total = self.count_a + self.count_t +self.count_g + self.count_c + self.count_n;
        self.rate_gc = (self.count_g + self.count_c) as f64 / total as f64;
        self.rate_n = self.count_n as f64 / total as f64;
    }

}


pub fn summary_fa(
    input: Vec<String>,
    all: bool,
) -> Result<(),Error> {
    if input.is_empty() {
        error!("usage: fakit  summ -h/--help, get more help");
        std::process::exit(1);
    }
    let start = Instant::now();

    if all {
        println!("file\tcount_A\tcount_C\tcount_G\tcount_T\tcount_N\trate_GC\trate_N\tnum_seq\tsum_len\tmin_len\tmean_len\tmax_len");
    } else {
        println!("file\tnum_seq\tsum_len\tmin_len\tmean_len\tmax_len");
    }

    for i in input {
        let mut info = Seqinfo::new(i.clone());
        let file_in = &Some(i.as_str());
        let mut min:Option<usize> = None;

        let fp = fasta::Reader::new(file_reader(file_in)?);
        for rec in fp.records().flatten() {
            let mut pos = 0usize;
            info.num_seq += 1;
            let seq_len = rec.seq().len();
            info.sum_len += seq_len;
            
            if seq_len > info.max_len {
                info.max_len = seq_len;    
            }
            min = if let Some(min) = min {
                if seq_len < min { Some(seq_len)} else { Some(min) }
            } else { 
                Some(seq_len)
            };
            for nt in rec.seq().iter() {
                pos += 1;
                match nt {
                    &b'A' | &b'a' => {info.count_a += 1;},
                    &b'T' | &b't' => {info.count_t += 1;},
                    &b'G' | &b'g' => {info.count_g += 1;},
                    &b'C' | &b'c' => {info.count_c += 1;},
                    &b'N' | &b'n' => {info.count_n += 1;},
                    _ => { 
                        warn!("Error DNA base code in sequence {} position: {}",rec.id(), pos);
                        continue;
                        //std::process::exit(1);
                    }
                }    
            }
        }
        info.min_len = min.unwrap();
        info.mean();
        info.rate();
        if all {
            println!("{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{}\t{}\t{}\t{:.0}\t{}",info.name, info.count_a, info.count_c, info.count_g, info.count_t, info.count_n, info.rate_gc, info.rate_n,
             info.num_seq, info.sum_len, info.min_len, info.mean_len, info.max_len);
        } else {
            println!("{}\t{}\t{}\t{}\t{:.0}\t{}",info.name, info.num_seq, info.sum_len, info.min_len, info.mean_len, info.max_len);
        }
    }

    info!("time elapsed is: {:?}",start.elapsed());
    Ok(())
}

