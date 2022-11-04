use clap::Parser;
use std::env;

mod fa;
mod utils;

use fa::*;

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.1",
    about = "fakit: a simple program for fasta file manipulation",
    long_about = None
)]
struct Args {
    /// input fasta[.gz] file.
    input: Option<String>,

    /// convert base to uppercase.
    #[arg(short = 'u', long = "upper")]
    upper: bool,

    /// convert base to lowercase.
    #[arg(short = 'l', long = "lower")]
    lower: bool,

    /// base number of each line, 0 for long single line.
    #[arg(short = 'w', long = "length")]
    len: Option<u64>,
    
    /// fasta to fastq and generate fake fastq quality.
    #[arg(short = 'F', long = "fake")]
    fake: Option<char>,  

    /// drop sequences with length shorter than int.
    #[arg(short = 'd', long = "drop")]
    drop: Option<u64>,

    /// r for reverse seq, m for match seq.
    #[arg(short = 'c', long = "convert")]
    conv: Option<String>,

    /// simple statistics of fasta file.
    #[arg(short = 's', long = "summary")]
    sum: bool,
}


fn main() {
    let args = Args::parse();
    let Args {
            input,
            upper,
            lower,
            len,
            fake,
            drop,
            conv,
            sum,
        } = args;

        let n = env::args().collect::<Vec<String>>().iter().len();
        if n < 3 {
            eprintln!("[info] error: type \"--help\"  for more information\n");
            std::process::exit(1);
        }
        if upper {
            let _ = upper_lower_fa(&input, true);
        } 
        if lower {
            let _ = upper_lower_fa(&input, false);
        }
        if len.is_some() {
            let _ = seq_len(&input, len);
        }
        if fake.is_some() {
            let _ = fake_quality(&input, fake);
        };
        if drop.is_some() {
            let _ = drop_short(&input, drop);
        }
        if conv.is_some() {
            let _ = rev_seq(&input, conv);
        }
        if sum {
            let _ = summary_fa(&input);
        }
   
}
