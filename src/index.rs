use std::collections::BTreeMap;
use std::io::{BufRead ,Result};

use crate::utils::*;


pub fn index_fasta(
    name: &Option<&str>,
    out: &Option<&str>,
) -> Result<()> {
    let fp = file_reader(name)?;
    let mut fo = file_writer(out)?;

    let mut dat = BTreeMap::new(); // base count
    let mut chr = BTreeMap::new(); // base indent
    let mut seq_id = String::new();
    let mut lens = vec![];
    let (mut dist, mut n, mut total_len)= (0, 0, 0);

    for line in fp.lines().flatten() {
        if line.starts_with('>') {
            n += 1;
            let head = line.split_whitespace().collect::<Vec<&str>>()[0];
            let id = head.strip_prefix('>').unwrap().to_string();
            
            dist += id.len() + 2;
            chr.insert(id.clone(),dist);
            if total_len != 0 {
                dat.insert(seq_id.clone(),total_len);
            } 
            seq_id = id;
            total_len = 0;
        } else {
            let len = line.len();
            if n == 1{
                n = 0;
                lens.push((len,len+1));
            }
            total_len += len;
            dist += len + 1
        }
    }
    // for last loop result
    dat.insert(seq_id.clone(),total_len);

    for (i,(k,v)) in dat.iter().enumerate() {
        let out = format!("{}\t{}\t{}\t{}\t{}",k,v,chr.get(k).unwrap(),lens[i].0, lens[i].1);    
        writeln!(&mut fo, "{}",out)?;
    }
    Ok(())
}
