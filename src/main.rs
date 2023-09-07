use clap::{Parser, Subcommand};
use anyhow::{Error, Ok};
use env_logger::Env;

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
mod utils;

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.2.6",
    about = "fakit: a simple program for fasta file manipulation",
    long_about = None,
    next_line_help = false
)]
struct Args {
    #[clap(subcommand)]
    command: Subcli,
}

#[derive(Subcommand, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
enum Subcli {
    /// get first N records from fasta file
    topn {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// print first N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10)]
        num: usize,
        /// output fasta[.gz] file name, or write to stdout
        output: Option<String>,
    }, 
    /// convert fasta to fastq file
    fa2fq {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'q', long = "qual", default_value_t = 'F')]
        qual: char,
        /// output fastq file name[.gz], or write to stdout
        output: Option<String>,
    },
    /// crate index and random access to fasta files
    faidx {
        /// input uncompressed fasta file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// fasta region format and start is 1-based, eg. chr1:1-5000 chr2:100-800
        /// usage:
        ///     fakit faidx -i seq.fa chr1:1-5000 chr2:100-800 ...
        #[arg(verbatim_doc_comment)]
        region: Option<Vec<String>>,
    },
    /// re-length fasta sequence 
    relen {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// specify each seq length, 0 for a single line  
        #[arg(short = 'l', long = "len", default_value_t = 70)]
        len: usize,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    }, 
    /// rename sequence id in fasta file
    rename {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// if specified, keep sequence id description
        #[arg(short = 'k', long = "keep")]
        keep: bool, 
        /// set new id prefix for sequence
        #[arg(short = 'p', long = "prefix")]
        prefix: Option<String>,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// stat dna fasta gc content by sliding windows
    window {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// set sliding window size
        #[arg(short = 'w', long = "window", default_value_t = 500)]
        wind: usize,
        /// set sliding window step size
        #[arg(short= 's', long = "step", default_value_t = 100)]
        step: usize,
        /// if specified, keep fasta format in output result
        #[arg(short = 'k', long = "keep")]
        keep: bool,
        /// output result[.gz] file name, or write to stdout
        /// header format: seqid    start   end gc_rate sequence
        #[arg(short = 'o', long = "out",verbatim_doc_comment )]
        output: Option<String>,
    },
    /// search subsequences/motifs from fasta file
    search {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// specify uppercase pattern/motif, e.g., -p "ATC{2,}" or -p ATCCG
        #[arg(short = 'p', long = "pattern")]
        pat: String,
        /// if specified, show header in result
        #[arg(short = 'H', long = "header")]
        Header: bool,
        /// output search result[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out",verbatim_doc_comment )]
        output: Option<String>,
    },
    /// subsample sequences from big fasta file
    subfa {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// set rand seed
        #[arg(short = 's', long = "seed", default_value_t = 69)]
        seed: u64,
        /// reduce much memory but cost more time
        #[arg(short = 'r', long = "rdc")]
        rdc: bool,
        /// subseq number
        #[arg(short = 'n', long = "num")]
        num: usize,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// split fasta file by sequence id
    split {
        /// input fasta[.gz] file
        #[arg(short = 'i', long = "input")]
        input: String,
        /// set file extension, eg. fa, fa.gz, fna, fna.gz
        #[arg(short = 'e', long = "ext")]
        ext: String,
        /// split fasta file output dir, default: current dir
        #[arg(short = 'o', long = "outdir")]
        outdir: Option<String>,
    },
    /// a simple summary for DNA fasta files
    summ {
        /// files to process, eg. *.fasta
        /// usage:
        ///     fakit summ *.fa[.gz]
        ///     fakit summ  query.fa tmp.fasta demo.fa.gz --all
        #[arg(verbatim_doc_comment)]
        file: Vec<String>,
        /// if specified, show more information
        #[arg(short='a', long="all")]
        all: bool,
    }
}

fn main() -> Result<(),Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    match args.command {
        Subcli::topn { input, num, output } => {
            if let Some(output) = output {
                top_n_records(num, &Some(&input), &Some(&output))?;
            } else {
                top_n_records(num, &Some(&input), &None)?;
            }
        }
        Subcli::fa2fq { input, qual, output } => {
            if let Some(output) = output {
                fake_quality(&Some(&input), qual, &Some(&output))?;
            } else {
                fake_quality(&Some(&input), qual, &None)?;
            }
        }
        Subcli::faidx { input, region } => {
            if let Some(region) = region {
                index_reader(&input, region)?;
                
            } else {
                index_fasta(&input)?;
            }
        }
        Subcli::relen { input, len, output } => {
            if let Some(output) = output {
                relen_fa(&Some(&input), len, &Some(&output))?;
            } else {
                relen_fa(&Some(&input), len, &None)?;
            }
        }
        Subcli::rename { input, keep, prefix, output} => {
            if let Some(output) = output {
                rename_fa(&Some(&input), keep, prefix, &Some(&output))?;
            } else {
                rename_fa(&Some(&input), keep, prefix, &None)?;
            }
        }
        Subcli::window { input, wind, step, keep, output } => {
            if let Some(output) = output {
                silding_window(step, wind, &input, &Some(&output), keep)?;
            } else {
                silding_window(step, wind, &input, &None, keep)?;
            }
        }
        Subcli::search { input, pat, Header, output } => {
            if let Some(output) = output {
                search_fa(&input, &Some(&output), &pat ,Header)?;
            } else {
                search_fa(&input, &None, &pat, Header)?;
            }
        }
        Subcli::subfa { input, seed, num, rdc, output} => {
            if rdc {
                if let Some(out) = output {
                    select_fasta(&Some(&input), num, seed, &Some(&out))?;
                } else {
                    select_fasta(&Some(&input), num, seed, &None)?;
                }
            } else {
                if let Some(out) = output {
                    select_fasta2(&Some(&input), num, seed, &Some(&out))?;
                } else {
                    select_fasta2(&Some(&input), num, seed, &None)?;
                }
            }
        }
        Subcli::summ { file ,all} => {
            summary_fa(file, all)?;
        }
        Subcli::split { input, ext, outdir } => {
            if let Some(outdir) = outdir {
                split_fa(input, ext, Some(&outdir))?;
            } else {
                split_fa(input, ext, None)?;
            }
        }
    }

    Ok(())
}
