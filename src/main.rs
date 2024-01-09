use anyhow::{Ok,Error};
use clap::Parser;
use log::error;

mod cli;
use cli::*;
mod logger;
use logger::*;
mod flatten;
use flatten::*;
mod shuffle;
use shuffle::*;
mod range;
use range::*;
mod reverse;
use reverse::*;
mod sort;
use sort::*;
mod seq;
use seq::*;
mod top;
use top::*;
mod fa2fq;
use fa2fq::*;
mod faidx;
use faidx::*;
mod relen;
use relen::*;
mod rename;
use rename::*;
mod search;
use search::*;
mod slide;
use slide::*;
mod subfa;
use subfa::*;
mod summ;
use summ::*;
mod split;
use split::*;
mod codon;
use codon::*;
mod utils;


fn main() -> Result<(), Error> {

    let args = cli::Args::parse();
    match args.logfile {
        Some(v) => logger(args.verbose, &Some(&v), args.quiet)?,
        None => logger(args.verbose, &None, args.quiet)?
    }

    match args.command {
        Subcli::topn { input, num, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    top_n_records(num, &Some(&input), &Some(&output), args.compression_level)?;
                } else {
                    top_n_records(num, &Some(&input), &None, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    top_n_records(num, &None, &Some(&output), args.compression_level)?;
                } else {
                    top_n_records(num, &None, &None, args.compression_level)?;
                }
            } 
        }
        Subcli::fa2fq { input, qual, output } => {
           if let Some(input) = input {
                if let Some(output) = output {
                    fake_quality(&Some(&input), qual, &Some(&output), args.compression_level)?;
                } else {
                    fake_quality(&Some(&input), qual, &None, args.compression_level)?;
                }
           } else {
                if let Some(output) = output {
                    fake_quality(&None, qual, &Some(&output), args.compression_level)?;
                } else {
                    fake_quality(&None, qual, &None, args.compression_level)?;
                }
           }
        }
        Subcli::faidx { input, region} => {
            if let Some(input) = input {
                if let Some(region) = region {
                    index_reader(&input, region, args.compression_level)?;
                } else {
                    index_fasta(&Some(&input), args.compression_level)?;
                }
            } else {
                error!("usage: fakit faidx -h/--help, get more help");
                std::process::exit(1);
            }
        }
        Subcli::flatten { input, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    flatten_fa(&Some(&input), &Some(&output), args.compression_level)?;
                } else {
                    flatten_fa(&Some(&input), &None, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    flatten_fa(&None, &Some(&output), args.compression_level)?;
                } else {
                    flatten_fa(&None, &None, args.compression_level)?;
                }
            }
        }
        Subcli::range { input, skip, take, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    range_fasta(&Some(&input), skip, take, &Some(&out), args.compression_level)?;
                } else {
                    range_fasta(&Some(&input), skip, take, &None, args.compression_level)?;
                }
            } else {
                if let Some(out) = out {
                    range_fasta(&None, skip, take, &Some(&out), args.compression_level)?;
                } else {
                    range_fasta(&None, skip, take, &None, args.compression_level)?;
                }
            }  
        }
        Subcli::relen { input, len, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    relen_fa(&Some(&input), len, &Some(&output),args.compression_level)?;
                } else {
                    relen_fa(&Some(&input), len, &None, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    relen_fa(&None, len, &Some(&output),args.compression_level)?;
                } else {
                    relen_fa(&None, len, &None, args.compression_level)?;
                }
            }
        }
        Subcli::rename { input, keep, prefix, output} => {
            if let Some(input) =input {
                if let Some(output) = output {
                    rename_fa(&Some(&input), keep, prefix, &Some(&output), args.compression_level)?;
                } else {
                    rename_fa(&Some(&input), keep, prefix, &None, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    rename_fa(&None, keep, prefix, &Some(&output), args.compression_level)?;
                } else {
                    rename_fa(&None, keep, prefix, &None, args.compression_level)?;
                }
            }
        }
        Subcli::reverse { input, rev, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    reverse_comp_seq(&Some(&input), &Some(&out), rev, args.compression_level)?;
                } else {
                    reverse_comp_seq(&Some(&input), &None, rev, args.compression_level)?;
                }   
            } else {
                if let Some(out) = out {
                    reverse_comp_seq(&None, &Some(&out), rev, args.compression_level)?;
                } else {
                    reverse_comp_seq(&None, &None, rev, args.compression_level)?;
                }
            }
        }
        Subcli::window { input, wind, step, keep, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    silding_window(step, wind, &Some(&input), &Some(&output), keep, args.compression_level)?;
                } else {
                    silding_window(step, wind, &Some(&input), &None, keep, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    silding_window(step, wind, &None, &Some(&output), keep, args.compression_level)?;
                } else {
                    silding_window(step, wind, &None, &None, keep, args.compression_level)?;
                }
            }
        }
        Subcli::seq { input, lower, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    seq_fa(&Some(&input), lower, &Some(&out), args.compression_level)?;
                } else {
                    seq_fa(&Some(&input), lower, &None, args.compression_level)?;
                }
            } else {
                if let Some(out) = out {
                    seq_fa(&None, lower, &Some(&out), args.compression_level)?;
                } else {
                    seq_fa(&None, lower, &None, args.compression_level)?;
                }
            }
        }
        Subcli::sort { input, name, seq, gc, length, reverse, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    sort_fasta(&Some(&input), name, seq, gc, length, reverse, &Some(&out), args.compression_level)?;
                } else {
                    sort_fasta(&Some(&input), name, seq, gc, length, reverse, &None, args.compression_level)?;
                }
            } else {
                if let Some(out) = out {
                    sort_fasta(&None, name, seq, gc, length, reverse, &Some(&out), args.compression_level)?;
                } else {
                    sort_fasta(&None, name, seq, gc, length, reverse, &None, args.compression_level)?;
                }
            }
        }
        Subcli::shuffle { input, seed, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    shuffle_fasta(&Some(&input), seed, &Some(&out), args.compression_level)?;
                } else {
                    shuffle_fasta(&Some(&input), seed, &None, args.compression_level)?;
                }
            } else {
                if let Some(out) = out {
                    shuffle_fasta(&None, seed, &Some(&out), args.compression_level)?;
                } else {
                    shuffle_fasta(&None, seed, &None, args.compression_level)?;
                }
            }
        }
        Subcli::search { input, pat, Header, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    search_fa(&Some(&input), &Some(&output), &pat ,Header, args.compression_level)?;
                } else {
                    search_fa(&Some(&input), &None, &pat, Header, args.compression_level)?;
                }
            } else {
                if let Some(output) = output {
                    search_fa(&None, &Some(&output), &pat ,Header, args.compression_level)?;
                } else {
                    search_fa(&None, &None, &pat, Header, args.compression_level)?;
                }
            }
        }
        Subcli::subfa { input, seed, num, rdc, output} => {
            if rdc {
                if let Some( input) = input {
                    if let Some(out) = output {
                        select_fasta(&Some(&input), num, seed, &Some(&out), args.compression_level)?;
                    } else {
                        select_fasta(&Some(&input), num, seed, &None, args.compression_level)?;
                    }
                } else {
                    if let Some(out) = output {
                        select_fasta(&None, num, seed, &Some(&out), args.compression_level)?;
                    } else {
                        select_fasta(&None, num, seed, &None, args.compression_level)?;
                    }
                }
            } else {
                if let Some(input) = input {
                    if let Some(out) = output {
                        select_fasta2(&Some(&input), num, seed, &Some(&out), args.compression_level)?;
                    } else {
                        select_fasta2(&Some(&input), num, seed, &None, args.compression_level)?;
                    }
                } else {
                    if let Some(out) = output {
                        select_fasta2(&None, num, seed, &Some(&out), args.compression_level)?;
                    } else {
                        select_fasta2(&None, num, seed, &None, args.compression_level)?;
                    }
                }
            }
        }
        Subcli::summ { file ,all} => {
            summary_fa(file, all)?;
        }
        Subcli::split { input, ext, outdir } => {
            if let Some(input) = input {
                if let Some(outdir) = outdir {
                    split_fa(&Some(&input), ext, Some(&outdir), args.compression_level)?;
                } else {
                    split_fa(&Some(&input), ext, None, args.compression_level)?;
                }
            } else {
                if let Some(outdir) = outdir {
                    split_fa(&None, ext, Some(&outdir), args.compression_level)?;
                } else {
                    split_fa(&None, ext, None, args.compression_level)?;
                }
            }
        }
        Subcli::codon { name } => {
            show_codon(name)?;
        }
    }
    
    Ok(())
}