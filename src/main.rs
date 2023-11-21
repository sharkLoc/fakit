use std::io::Write;
use clap::{Parser, Subcommand};
use anyhow::{Error, Ok};
use chrono::Local;
use env_logger::{Builder,fmt::Color};
use log::{LevelFilter,Level, error};



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

#[derive(Parser, Debug)]
#[command(
    author = "sharkLoc",
    version = "0.2.9",
    about = "A simple program for fasta file manipulation",
    long_about = None,
    next_line_help = false,
    before_help = None,
    help_template = "{name}: {about}\n\nVersion: {version}\
    \nAuthors: {author} <mmtinfo@163.com>\
    \n\n{usage-heading} {usage}\n\n{all-args}\n"
)]
struct Args {
    #[clap(subcommand)]
    command: Subcli,
    /// be quiet and do not show extra information
    #[arg(short = 'q', long = "quiet", global= true, help_heading = Some("Global FLAGS"))]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
enum Subcli {
    /// Get first N records from fasta file
    topn {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// print first N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10)]
        num: usize,
        /// output fasta[.gz] file name, or write to stdout
        output: Option<String>,
    }, 
    /// Convert fasta to fastq file
    fa2fq {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'q', long = "qual", default_value_t = 'F')]
        qual: char,
        /// output fastq file name[.gz], or write to stdout
        output: Option<String>,
    },
    /// Create index and random access to fasta files
    #[command(visible_alias="fai")]
    faidx {
        /// input uncompressed fasta file
        input: Option<String>,
        /// fasta region format and start is 1-based, eg. chr1:1-5000 chr2:100-800
        /// usage:  fakit faidx seq.fa chr1:1-5000 chr2:100-800 ...
        #[arg(verbatim_doc_comment)]
        region: Option<Vec<String>>,
    },
    /// Re-length fasta sequence 
    relen {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// specify each seq length, 0 for a single line  
        #[arg(short = 'l', long = "len", default_value_t = 70)]
        len: usize,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    }, 
    /// Rename sequence id in fasta file
    rename {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// if specified, keep sequence id description
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool, 
        /// set new id prefix for sequence
        #[arg(short = 'p', long = "prefix")]
        prefix: Option<String>,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// Stat dna fasta gc content by sliding windows
    window {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// set sliding window size
        #[arg(short = 'w', long = "window", default_value_t = 500)]
        wind: usize,
        /// set sliding window step size
        #[arg(short= 's', long = "step", default_value_t = 100)]
        step: usize,
        /// if specified, keep fasta format in output result
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool,
        /// output result[.gz] file name, or write to stdout
        /// header format: seqid    start   end gc_rate sequence
        #[arg(short = 'o', long = "out",verbatim_doc_comment )]
        output: Option<String>,
    },
    /// Search subsequences/motifs from fasta file
    search {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// specify uppercase pattern/motif, e.g., -p "ATC{2,}" or -p ATCCG
        /// search multiple pattern/motif, -p "ATCCG|GCTAA"
        #[arg(short = 'p', long = "pattern",verbatim_doc_comment)]
        pat: String,
        /// if specified, show header in result
        #[arg(short = 'H', long = "header", help_heading = Some("FLAGS"))]
        Header: bool,
        /// output search result[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out" )]
        output: Option<String>,
    },
    /// Subsample sequences from big fasta file
    subfa {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// set rand seed
        #[arg(short = 's', long = "seed", default_value_t = 69)]
        seed: u64,
        /// reduce much memory but cost more time
        #[arg(short = 'r', long = "rdc", help_heading=Some("FLAGS"))]
        rdc: bool,
        /// subseq number
        #[arg(short = 'n', long = "num")]
        num: usize,
        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// Split fasta file by sequence id
    split {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// set output file extension, eg. fa, fa.gz, fna, fna.gz
        #[arg(short = 'e', long = "ext")]
        ext: String,
        /// split fasta file output dir, default: current dir
        #[arg(short = 'o', long = "outdir")]
        outdir: Option<String>,
    },
    /// A simple summary for DNA fasta files
    summ {
        /// files to process, eg. *.fasta
        /// usage:  fakit summ *.fa[.gz]
        /// usage:  fakit summ  query.fa tmp.fasta demo.fa.gz --all
        #[arg(verbatim_doc_comment)]
        file: Vec<String>,
        /// if specified, show more information
        #[arg(short='a', long="all", help_heading=Some("FLAGS"))]
        all: bool,
    },
    /// Show codon table and amino acid name
    codon {
        /// Amino acid short name eg. S
        #[arg(short='n', long="name")]
        name: Option<String>,
    }
}

fn main() -> Result<(),Error> {

    let mut builder = Builder::from_default_env();

    builder.format(|buf, record| {
        let mut style = buf.style();
        match record.level() {
            Level::Error => {
                style.set_color(Color::Red).set_bold(true);
            }
            Level::Warn => {
                style.set_color(Color::Yellow).set_bold(true);
            }
            Level::Info => {
                style.set_color(Color::Green).set_bold(true);
            }
            Level::Debug => {
                style.set_color(Color::Blue).set_bold(true);
            }
            Level::Trace => {
                style.set_color(Color::Magenta).set_bold(true);
            }
        }
        writeln!(buf,
            "[{} {} - {}] {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            style.value(record.level()),
            buf.style().set_color(Color::Rgb(90, 150, 150)).value(record.target()),
            record.args()
        )
    })
    .filter(None, LevelFilter::Info)
    .init();

    let args = Args::parse();

    match args.command {
        Subcli::topn { input, num, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    top_n_records(num, &Some(&input), &Some(&output), args.quiet)?;
                } else {
                    top_n_records(num, &Some(&input), &None, args.quiet)?;
                }
            } else {
                if let Some(output) = output {
                    top_n_records(num, &None, &Some(&output), args.quiet)?;
                } else {
                    top_n_records(num, &None, &None, args.quiet)?;
                }
            } 
        }
        Subcli::fa2fq { input, qual, output } => {
           if let Some(input) = input {
                if let Some(output) = output {
                    fake_quality(&Some(&input), qual, &Some(&output), args.quiet)?;
                } else {
                    fake_quality(&Some(&input), qual, &None,args.quiet)?;
                }
           } else {
            if let Some(output) = output {
                fake_quality(&None, qual, &Some(&output), args.quiet)?;
            } else {
                fake_quality(&None, qual, &None, args.quiet)?;
            }
           }
        }
        Subcli::faidx { input, region } => {
            if let Some(input) = input {
                if let Some(region) = region {
                    index_reader(&input, region, args.quiet)?;
                } else {
                    index_fasta(&Some(&input), args.quiet)?;
                }
            } else {
                error!("can't crate faidx for stdin stream");
                std::process::exit(1);
            }
        }
        Subcli::relen { input, len, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    relen_fa(&Some(&input), len, &Some(&output),args.quiet)?;
                } else {
                    relen_fa(&Some(&input), len, &None, args.quiet)?;
                }
            } else {
                if let Some(output) = output {
                    relen_fa(&None, len, &Some(&output),args.quiet)?;
                } else {
                    relen_fa(&None, len, &None, args.quiet)?;
                }
            }
        }
        Subcli::rename { input, keep, prefix, output} => {
            if let Some(input) =input {
                if let Some(output) = output {
                    rename_fa(&Some(&input), keep, prefix, &Some(&output), args.quiet)?;
                } else {
                    rename_fa(&Some(&input), keep, prefix, &None, args.quiet)?;
                }
            } else {
                if let Some(output) = output {
                    rename_fa(&None, keep, prefix, &Some(&output), args.quiet)?;
                } else {
                    rename_fa(&None, keep, prefix, &None, args.quiet)?;
                }
            }
        }
        Subcli::window { input, wind, step, keep, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    silding_window(step, wind, &Some(&input), &Some(&output), keep, args.quiet)?;
                } else {
                    silding_window(step, wind, &Some(&input), &None, keep, args.quiet)?;
                }
            } else {
                if let Some(output) = output {
                    silding_window(step, wind, &None, &Some(&output), keep, args.quiet)?;
                } else {
                    silding_window(step, wind, &None, &None, keep, args.quiet)?;
                }
            }
        }
        Subcli::search { input, pat, Header, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    search_fa(&Some(&input), &Some(&output), &pat ,Header, args.quiet)?;
                } else {
                    search_fa(&Some(&input), &None, &pat, Header, args.quiet)?;
                }
            } else {
                if let Some(output) = output {
                    search_fa(&None, &Some(&output), &pat ,Header, args.quiet)?;
                } else {
                    search_fa(&None, &None, &pat, Header, args.quiet)?;
                }
            }
        }
        Subcli::subfa { input, seed, num, rdc, output} => {
            if rdc {
                if let Some( input) = input {
                    if let Some(out) = output {
                        select_fasta(&Some(&input), num, seed, &Some(&out), args.quiet)?;
                    } else {
                        select_fasta(&Some(&input), num, seed, &None, args.quiet)?;
                    }
                } else {
                    if let Some(out) = output {
                        select_fasta(&None, num, seed, &Some(&out), args.quiet)?;
                    } else {
                        select_fasta(&None, num, seed, &None, args.quiet)?;
                    }
                }
            } else {
                if let Some(input) = input {
                    if let Some(out) = output {
                        select_fasta2(&Some(&input), num, seed, &Some(&out), args.quiet)?;
                    } else {
                        select_fasta2(&Some(&input), num, seed, &None, args.quiet)?;
                    }
                } else {
                    if let Some(out) = output {
                        select_fasta2(&None, num, seed, &Some(&out), args.quiet)?;
                    } else {
                        select_fasta2(&None, num, seed, &None, args.quiet)?;
                    }
                }
            }
        }
        Subcli::summ { file ,all} => {
            summary_fa(file, all, args.quiet)?;
        }
        Subcli::split { input, ext, outdir } => {
            if let Some(input) = input {
                if let Some(outdir) = outdir {
                    split_fa(&Some(&input), ext, Some(&outdir), args.quiet)?;
                } else {
                    split_fa(&Some(&input), ext, None, args.quiet)?;
                }
            } else {
                if let Some(outdir) = outdir {
                    split_fa(&None, ext, Some(&outdir), args.quiet)?;
                } else {
                    split_fa(&None, ext, None, args.quiet)?;
                }
            }
        }
        Subcli::codon { name } => {
            show_codon(name)?;
        }
    }

    Ok(())
}
