use bio::io::fasta::{self, Record};
use std::io::Result;

use crate::utils::*;


pub fn rename_fa(
    input: &Option<&str>,
    keep: bool,
    prefix: Option<String>, //&str,
    output: &Option<&str>,
) -> Result<()> {
    let fp = fasta::Reader::new(file_reader(input)?);
    let mut fo = fasta::Writer::new(file_writer(output)?);
    let mut n: usize = 0;

    if let Some(pre) = prefix {
        for rec in fp.records().flatten() {
            n += 1;
            let newid = format!("{}{}",pre,n);
            let record = if keep { 
                Record::with_attrs(&newid, rec.desc(), rec.seq()) 
            } else { 
                Record::with_attrs(&newid, None, rec.seq())
            };
            fo.write_record(&record)?;
        }
    } else {
        for rec in fp.records().flatten() {
            n += 1;
            let record = if keep { 
                Record::with_attrs(rec.id(), rec.desc(), rec.seq()) 
            } else { 
                Record::with_attrs(rec.id(), None, rec.seq())
            };
            fo.write_record(&record)?;
        }
    }
    
    /*for rec in fp.records().flatten() {
        n += 1;
        let newid = format!("{}{}",prefix,n);
        let record = if keep { 
            Record::with_attrs(&newid, rec.desc(), rec.seq()) 
        } else { 
            Record::with_attrs(&newid, None, rec.seq())
        };
        fo.write_record(&record)?;
        
    }*/

    Ok(())
}