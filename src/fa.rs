
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    io::{
        Result,
        BufRead
    },
};


use crate::utils::*;


pub fn upper_lower_fa(name: &Option<String>, flag: bool) -> Result<()> {
   
    let fp = file_reader(name)?;
    if flag {
        for line in fp.lines().flatten() {
            if line.starts_with('>'){
                println!("{}",line);
            } else {
                println!("{}",line.to_uppercase());
            } 
        }
    } else {
        for line in fp.lines().flatten() {
            if line.starts_with('>'){
                println!("{}",line);
            } else {
                println!("{}",line.to_lowercase());
            } 
        }
    }
    Ok(())
}


pub fn seq_len(name: &Option<String>, w: Option<u64>) -> Result<()> {

    let fp = file_reader(name)?;
    if let Some(w) = w {
        if w == 0 {
            let mut flag: bool = false;
            for line in fp.lines().flatten() {
                if line.starts_with('>'){
                    if flag {
                        println!();  
                    } else {
                        flag = true;
                    }
                    println!("{}",line);
                } else {
                    print!("{}",line);
                } 
            }
            println!()
        } else {
            let mut seq = BTreeMap::new();
            let mut id = String::new();
            for line in fp.lines().flatten() {
                if line.starts_with('>') {
                    id = line.strip_prefix('>').unwrap().to_string();
                    seq.insert(id.clone(), String::from(""));
                } else {
                    seq.entry(id.clone()).or_insert_with(|| "".to_string()).push_str(line.as_str());                      
                }
            }
            // keep raw order 
            for (k,v) in seq {
                let mut n  = 0u64;
                let len = v.len() as u64;
                println!(">{}",k);
                for nt in v.chars() {
                    n += 1;
                    print!("{}",nt);
                    if n == w {
                            println!();
                        n = 0;
                    }
                }
                if len % w != 0 {
                    println!();
                }
            }
        }
    }
    Ok(())
}


pub fn fake_quality(name: &Option<String>, q: Option<char>) -> Result<()> {

    let fp = file_reader(name)?;
    let mut seq = BTreeMap::new();
    let mut id = String::new();
    for tx in fp.lines().flatten() {
        if tx.starts_with('>') {
            id = tx.replace('>', "@");
            seq.insert(id.clone(), String::from(""));
        } else {
            seq.entry(id.clone()).or_insert_with(|| "".to_string()).push_str(tx.as_str());                        
        }
    }
    for (k,v) in seq {
        println!("{}\n{}\n+\n{}",k,v,q.unwrap().to_string().repeat(v.len()));
    }
    Ok(())
}


pub fn drop_short(name: &Option<String>, ds: Option<u64>) -> Result<()> {
    let fp = file_reader(name)?;
    if let Some(d) = ds {
        let mut seq = BTreeMap::new();
        let mut id = String::new();
        for tx in fp.lines().flatten() {
            if tx.starts_with('>') {
                id = tx;
                seq.insert(id.clone(), String::from(""));
            } else {
                seq.entry(id.clone()).or_insert_with(|| "".to_string()).push_str(tx.as_str());                        
            }
        }
        // filter 
        for (k,v) in seq {
            if v.len() as u64 >= d {
                println!("{}\n{}",k,v);
            }
        }
    }
    Ok(())
}


pub fn rev_seq(name: &Option<String>, conv: Option<String>) -> Result<()>{
    let fp = file_reader(name)?;
    let mut seq = BTreeMap::new();
    let mut id = String::new();
    for tx in fp.lines().flatten() {
        if tx.starts_with('>') {
            id = tx;
            seq.insert(id.clone(), String::from(""));
        } else {
            seq.entry(id.clone()).or_insert_with(|| "".to_string()).push_str(tx.as_str());                        
        }
    }
    match conv {
        Some(x) => {
            if x == *"r" {
                for (k,v) in &seq {
                    println!("{}\n{}", k, v.chars().rev().collect::<String>());
                }
            } else if x == *"m" {
                let map_ha =HashMap::from([
                    ('A','T'), ('G','C'), ('C','G'), ('T','A'), ('N','N'),
                    ('a','t'), ('g','c'), ('c','g'), ('t','a'), ('n','n')]);
        
                for (k,v) in &seq { 
                    println!("{}",k);
                    for x in v.chars() {
                        print!("{}",map_ha.get(&x).unwrap());
                    }
                    println!();
                }
            } else {
                eprintln!("[info] error: invaliad args");
                eprintln!("[info] info: use -c r or -c w");
                std::process::exit(1);
            }
        },
        None => {
            unreachable!();
        }
    }
    Ok(())
}

#[allow(non_snake_case)]
struct Iterm {
    id: String,
    A: usize,
    T: usize,
    G: usize,
    C: usize,
    N: usize,
    GC: f64,
    len: usize
}

impl Iterm {
    fn new() -> Self {
        Iterm { id: String::new(), A: 0, T: 0, G: 0, C: 0, N: 0, GC: 0.0, len: 0 }
    }
    fn gc(&self) -> f64 {
        (self.C + self.G) as f64 / self.len as f64 
    }
    fn show(&self) {
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{}",self.id,self.A,self.T,self.G, self.C, self.N, self.GC, self.len);
    }
}

#[allow(non_camel_case_types)]
type count_nt = (usize,usize,usize,usize,usize);

#[inline]
fn conut_seq(src: &str) -> count_nt {
    
    let mut ret: count_nt = (0,0,0,0,0);
    for x in src.as_bytes().iter() {
        match *x {
            b'A' => ret.0 += 1,
            b'T' => ret.1 += 1,
            b'G' => ret.2 += 1,
            b'C' => ret.3 += 1,
            b'N' => ret.3 += 1,
            _ => {
                eprintln!("[info] error: Error base code!");
                std::process::exit(1);
            }
        }
    }
    ret
}

pub fn summary_fa(name: &Option<String>) -> Result<()> {

    let fp = file_reader(name)?;
    let mut content: BTreeMap<String, Iterm> = BTreeMap::new();
    let mut id = String::new();

    for line in fp.lines().flatten() {
        if line.starts_with('>'){
            id = line.replace('>', "");
            content.insert(id.clone(), Iterm::new());
            content.get_mut(&id).unwrap().id = id.clone();
        } else {
            let seq = line.to_uppercase();
            let (nta, ntt, ntg, ntc, ntn) = conut_seq(&seq);
            let lens = nta + ntt + ntg + ntc + ntn;

            content.get_mut(&id).unwrap().A += nta;
            content.get_mut(&id).unwrap().T += ntt;
            content.get_mut(&id).unwrap().G += ntg;
            content.get_mut(&id).unwrap().C += ntc;
            content.get_mut(&id).unwrap().N += ntn;
            content.get_mut(&id).unwrap().len += lens;
        }
    }
    println!("id\tbase_A\tbase_T\tbase_G\tbase_C\tbase_N\tGC_Rate\tseq_Len");
    
    for info in content.values_mut() {
        info.GC = info.gc();
        info.show();
    }
    Ok(())
}




