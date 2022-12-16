use clap::Parser;

mod format;
mod index;
mod stats;
mod utils;

use format::*;
use index::*;
use stats::*;


#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.2.0",
    about = "fakit: a simple program for fasta file manipulation",
    long_about = None
)]

struct Args {
    #[clap(subcommand)]
    command: Subcli,
}

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
enum Subcli {
    /// summary fasta file
    summ {
        /// input fasta[.gz] file
        input: Option<String>,
    },
    /// state fasta file
    stats {
        /// input fasta.[gz] file
        input: Option<String>,
    },
    /// convert fasta to fastq file
    fa2fq {
        /// input fasta.[gz] file
        input: Option<String>,

        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'q', long = "qual", default_value_t = 'F')]
        qual: char,

        /// output file name[.gz], or write to stdout
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
    },
    /// crate index for fasta file
    faidx {
        /// input fasta file
        input: Option<String>,
    },
    /// format fasta file
    fmt {
        /// input fasta.[gz] file
        input: Option<String>,

        /// specify each seq length, 0 for a single line
        #[arg(short = 'l', long = "len", default_value_t = 70)]
        len: u64,

        /// output fasta[.gz] file name, or write to stdout
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Subcli::summ { input } => {
            if let Some(input) = input {
                let _x = summary_fa(&Some(input.as_str()));    
            } else {
                let _x = summary_fa(&None);
            }
        }
        Subcli::stats { input } => {
            if let Some(input) = input {
                let _x = stats_fa(&Some(input.as_str()));
            } else {
                let _x = stats_fa(&None);
            }
        }
        Subcli::fa2fq { input, qual, out } => {
            if let Some(input) = input {
                if let Some(out) = out {
                    let _x = fake_quality(&Some(&input), qual, &Some(&out));    
                } else {
                    let _x = fake_quality(&Some(&input), qual, &None);
                }    
            } else {
                if let Some(out) = out {
                    let _x = fake_quality(&None, qual, &Some(&out));    
                } else {
                    let _x = fake_quality(&None, qual, &None);
                }
            }
        }
        Subcli::faidx { input } => { 
            if let Some(file) = input {
                let _x = index_fasta(&Some(&file), &Some(format!("{}.fai",file.as_str()).as_str()) );
            } else {
                eprintln!("[error]: need specify a fasta file name");
                std::process::exit(1);
            }
        }
        Subcli::fmt { input, len, output } => {
            if let Some(input) = input {
                if let Some(output) = output {
                    let _x = format_fa(&Some(&input), len, &Some(&output));        
                } else {
                    let _x = format_fa(&Some(&input), len, &None);
                }
            } else {
                if let Some(output) = output {
                    let _x = format_fa(&None, len, &Some(&output));
                } else {
                    let _x = format_fa(&None, len, &None);
                }
            } 
        }
    }
}
