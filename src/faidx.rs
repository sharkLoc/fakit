use std::collections::BTreeMap;
use std::io::BufRead;
use crate::utils::*;
use bio::io::fasta::IndexedReader;
use anyhow::Error;

pub fn index_fasta(
    name: &str,
) -> Result<(), Error> {
    let out = format!("{}.fai",name);
    let fp = file_reader(&Some(name))?;
    let mut fo = file_writer(&Some(&out))?;

    let mut dat = BTreeMap::new(); // base count
    let mut chr = BTreeMap::new(); // base indent
    let mut seq_id = String::new();
    let mut each_len: Option<usize> = None;
    let mut lens = vec![];
    let (mut dist, mut total_len,mut order)= (0, 0, 0usize);
    let mut once = 0;

    for line in fp.lines().flatten() {
        order += 1;
        if line.starts_with('>') {
            let info = line.splitn(2,' ').collect::<Vec<&str>>();
            let desc_len = if info.len() == 2 { info[1].len()+ 1 } else { 0 };
            let id = info[0].strip_prefix('>').unwrap().to_string();
            
            dist += id.len() + desc_len + 2;  // +2 for ">" and "\n"
            chr.insert(id.clone(),dist);
            if total_len != 0 {
                dat.insert(seq_id.clone(),total_len);
            } 
            seq_id = id;
            total_len = 0;
            each_len = None;
            once += 1;
        } else {
            let len = line.len();
            if each_len.is_none() {
                each_len = Some(len);
            } 
            if let Some(each) = each_len {
                if len > each {
                    eprintln!("[error]: Different line length in line '{}'",order);
                    std::process::exit(1);
                }
            }
            if once == 1 {
                once = 0;
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

pub fn index_reader(
    name: &str,
    region: Vec<String>,
) -> Result<(),Error> {
    let fai = format!("{}.fai",name);
    if !std::path::Path::new(&fai).exists() {
        index_fasta(name)?;
    }
    let mut faidx = IndexedReader::from_file(&name).unwrap();
    for reg in &region {
        let each_reg: Vec<&str> = reg.split(|c| c == ':' || c == '-').collect();
        if each_reg.len() == 3 || each_reg.len() == 1{
            let start = each_reg[1].parse::<u64>()? - 1; //  start is 0-based, inclusive
        let end = each_reg[2].parse::<u64>()?;       //  stop is 0-based, exclusive
        if start > end {
            eprintln!("[warn]: Failed to fetch sequence in {}, skip",reg);
            continue;
        }
        // changge file pointer
        faidx.fetch(each_reg[0], start, end)?;

        let mut n = 0usize;
        println!(">{}",reg);
        for x in faidx.read_iter()? {
            n += 1;
            print!("{}",x? as char);
            if n == 70 {
                print!("\n");
                n = 0;
            }
        }
        let len = end - start ;
        if len % 70 != 0 { println!(); }
        } else {
            eprintln!("[warn]: Failed to parse region {}, skip",reg);
            continue;
        }
        
    }
    
    Ok(())
}