use anyhow::Error;
use regex::Regex;
use bio::io::fasta;
use log::*;
use crate::utils::*;


pub fn search_fa(
    file: &str,
    out: &Option<&str>,
    pat: &str,
    header: bool,
) -> Result<(),Error> {
    info!("reading from {}",file);
    info!("regex pattern {}",pat);
    let re = Regex::new(pat)?;
    let fp = file_reader(&Some(file)).map(fasta::Reader::new)?;
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
    Ok(())
}