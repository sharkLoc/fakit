use crate::utils::file_writer;
use anyhow::{Ok, Result};
use log::{error, info, warn};
use noodles::core::{position::Position, region::interval::Interval, Region};
use noodles::fasta::{self, fai, index, indexed_reader};
use std::path::PathBuf;
use std::{path::Path, time::Instant};

pub fn faidx_fasta<P: AsRef<Path> + Copy>(
    input: Option<P>,
    region: Option<Vec<String>>,
    output: Option<P>,
    compression_level: u32,
) -> Result<()> {
    let start = Instant::now();

    if let Some(file) = input {
        let fai = format!("{}.fai", file.as_ref().display());
        if PathBuf::from(fai.clone()).exists() {
            warn!("fasta index file is exists");
            if let Some(regs) =  region {
                let mut fa_index_reader =
                    indexed_reader::Builder::default().build_from_path(file)?;
                let mut wtr = file_writer(output, compression_level).map(fasta::Writer::new)?;
    
                for reg in regs {
                    let info = reg.split(':').collect::<Vec<&str>>();
                    let pos = info[1].split('-').collect::<Vec<&str>>();
                    info!(
                        "parse region {}, id: {}, start: {}, end: {}",
                        reg, info[0], pos[0], pos[1]
                    );
    
                    let start = Position::try_from(pos[0].parse::<usize>()?)?;
                    let end = Position::try_from(pos[1].parse::<usize>()?)?;
                    let reg_new = Region::new(info[0], Interval::from(start..=end));
                    let target = fa_index_reader.query(&reg_new)?;
                    wtr.write_record(&target)?;
                }
            }
            std::process::exit(1);
        }
        
        // faidx not exists 
        info!("create index file for: {}", file.as_ref().display());
        let fai_wtr = file_writer(Some(&fai), 0u32)?;
        // create index for fasta
        let index = index(input.unwrap())?;
        let mut faidx_wtr = fai::Writer::new(fai_wtr);
        faidx_wtr.write_index(&index)?;
        info!("index done, write index file to: {}", fai);

        // parse region
        if let Some(regs) =  region {
            let mut fa_index_reader =
                indexed_reader::Builder::default().build_from_path(file)?;
            let mut wtr = file_writer(output, compression_level).map(fasta::Writer::new)?;

            for reg in regs {
                let info = reg.split(':').collect::<Vec<&str>>();
                let pos = info[1].split('-').collect::<Vec<&str>>();
                info!(
                    "parse region {}, id: {}, start: {}, end: {}",
                    reg, info[0], pos[0], pos[1]
                );

                let start = Position::try_from(pos[0].parse::<usize>()?)?;
                let end = Position::try_from(pos[1].parse::<usize>()?)?;
                let reg_new = Region::new(info[0], Interval::from(start..=end));
                let target = fa_index_reader.query(&reg_new)?;
                wtr.write_record(&target)?;
            }
        }
    } else {
        error!("use opt -h get more help information");
        std::process::exit(1);
    }
    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
