use anyhow::Error;
use regex::Regex;
use bio::io::fasta;
use std::time::Instant;
use log::*;
use crate::utils::*;


pub fn search_fa(
    file: &Option<&str>,
    out: &Option<&str>,
    pat: &str,
    header: bool,
    quiet: bool,
) -> Result<(),Error> {
    if !quiet {
        if let Some(file) = file {
            info!("reading from file: {}",file);
        } else {
            info!("reading from stdin");
        }
        info!("regex pattern is: {}",pat);
    }
    
    let start = Instant::now();

    let re = Regex::new(pat)?;
    let fp = file_reader(file).map(fasta::Reader::new)?;
    let mut fo = file_writer(out)?;
    if header {
        fo.write("sequence_name\tstart\tend\tpattern\tlength\tsequence\n".as_bytes())?;
    }

    for rec in fp.records().flatten(){
        let seq = rec.seq().to_ascii_uppercase();
        let seq_str = std::str::from_utf8(seq.as_slice())?;
        let result = re.captures_iter(seq_str);
        for ret  in result {
            let group = ret.len();
            for i in 0..group {
                if let Some(x) = ret.get(i) {
                    let out_str = format!("{}\t{}\t{}\t{}\t{}\t{}\n",rec.id(), x.start()+1, x.end(), pat, x.end()-x.start(), x.as_str());
                    fo.write(out_str.as_bytes())?;
                }
            }
        }
    }

    if !quiet {
        info!("time elapsed is: {:?}",start.elapsed());
    }
    Ok(())
}