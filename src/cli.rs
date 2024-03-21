use clap::{Parser,value_parser};

#[derive(Parser, Debug)]
#[command(
    name = "Fakit",
    author = "sharkLoc",
    version = "0.3.0",
    about = "A simple program for fasta file manipulation",
    long_about = None,
    next_line_help = false,
    before_help = r"Fakit supports reading and writing gzip (.gz) format.
Bzip2 (.bz2) and xz (.xz) format is supported since v0.3.0.
Under the same compression level, xz has the highest compression ratio but consumes more time.

Compression level:
  format   range   default   crate
  gzip     1-9     6         https://crates.io/crates/flate2
  bzip2    1-9     6         https://crates.io/crates/bzip2
  xz       1-9     6         https://crates.io/crates/xz2",
    help_template = "{name}: {about}\n\nVersion: {version}\
    \n\nAuthors: {author} <mmtinfo@163.com>\
    \nSource code: https://github.com/sharkLoc/fakit.git\
    \n\n{before-help}
{usage-heading} {usage}\n\n{all-args}\n\nUse \"fakit help [command]\" for more information about a command"
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Subcli,
    /// line width when outputting fasta sequences, 0 for no wrap
    #[arg(short = 'w', long = "line-width", default_value_t = 70,
        global = true, value_name = "int",
        help_heading = Some("Global Arguments")
    )]
    pub width: usize,
    /// set gzip/bzip2/xz compression level 1 (compress faster) - 9 (compress better) for gzip/bzip2/xz output file,
    /// just work with option -o/--out
    #[arg(long = "compress-level", default_value_t = 6, global = true,
        value_parser = value_parser!(u32).range(1..=9), value_name = "int",
        help_heading = Some("Global Arguments")
    )]
    pub compression_level: u32,
    /// if file name specified, write log message to this file, or write to stderr
    #[arg(long = "log", global = true, help_heading = Some("Global Arguments"), value_name = "str")]
    pub logfile: Option<String>,
    /// control verbosity of logging, possible values: {error, warn, info, debug, trace}
    #[arg(short = 'v', long = "verbosity", global = true, value_name = "str",
        default_value_t = String::from("debug"),
        help_heading = Some("Global Arguments")
    )]
    pub verbose: String,
    /// be quiet and do not show extra information
    #[arg(short = 'q', long = "quiet", global= true, help_heading = Some("Global FLAGS"))]
    pub quiet: bool,
}

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum Subcli {
    /// get first N records from fasta file
    #[command(visible_alias = "head")]
    topn {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// print first N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10)]
        num: usize,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// convert fasta to fastq file
    fa2fq {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'Q', long = "qual", default_value_t = 'F')]
        qual: char,
        /// output fastq file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// create index and random access to fasta files
    #[command(visible_alias="fai")]
    faidx {
        /// input uncompressed fasta file
        input: Option<String>,
        /// fasta region format and start is 1-based, eg. chr1:1-5000 chr2:100-800
        /// usage:  fakit faidx seq.fa chr1:1-5000 chr2:100-800 ...
        #[arg(verbatim_doc_comment)]
        region: Option<Vec<String>>,
    }, 
    /// flatten fasta sequences
    #[command(visible_alias = "flat")]
    flatten {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if specified, keep sequence id description
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// print fasta records in a range
    range {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// skip first int fasta records
        #[arg(short = 's', long = "skip", default_value_t = 0)]
        skip: usize,
        /// take int fasta records
        #[arg(short = 't', long = "take")]
        take: usize,
        /// fasta output file name or write to stdout, files ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        out: Option<String>,

    },
    /// rename sequence id in fasta file
    rename {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if specified, keep sequence id description
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool, 
        /// set new id prefix for sequence
        #[arg(short = 'p', long = "prefix", value_name = "str")]
        prefix: Option<String>,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// get a reverse-complement of fasta file.
    #[command(visible_alias = "rev")]
    reverse {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if set, just output reverse sequences
        #[arg(short = 'r', long = "reverse", help_heading = Some("FLAGS"))]
        rev: bool,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        out: Option<String>,
    },
    /// stat dna fasta gc content by sliding windows
    #[command(visible_alias = "slide")]
    window {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set sliding window size
        #[arg(short = 'W', long = "window-size", default_value_t = 500, value_name = "int")]
        wind: usize,
        /// set sliding window step size
        #[arg(short= 's', long = "step-size", default_value_t = 100, value_name = "int")]
        step: usize,
        /// if specified, keep fasta format in output result
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool,
        /// output result file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        ///header format: seqid    start   end gc_rate sequence
        #[arg(short = 'o', long = "out",verbatim_doc_comment, value_name = "str")]
        output: Option<String>,
    }, 
    /// convert all bases to lower/upper case
    seq {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if specified, convert all bases to lowercase
        #[arg(short = 'l', long = "lower-case")]
        lower: bool,
        /// if specified, convert all bases to uppercase
        #[arg(short = 'u', long = "upper-case")]
        upper: bool,
        /// fasta sequences shorter than length required will be discarded
        #[arg(short = 'm', long = "min-len", value_name = "int")]
        min: Option<usize>,
        /// fasta sequences longer than length required will be discarded
        #[arg(short = 'M', long = "max-len", value_name = "int")]
        max: Option<usize>,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        out: Option<String>,
    },
    /// sort fasta file by name/seq/gc/length
    #[command(before_help = "note: all records will be readed into memory")]
    sort {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// sort sequences by name
        #[arg(short = 'n', long = "sort-by-name" ,help_heading = Some("FLAGS"))]
        name: bool,
        /// sort sequences by sequence
        #[arg(short = 's', long = "sort-by-seq" ,help_heading = Some("FLAGS"))]
        seq: bool,
        /// sort sequences by gc content
        #[arg(short = 'g', long = "sort-by-gc", help_heading = Some("FLAGS"))]
        gc: bool,
        /// sort sequences by length
        #[arg(short = 'l', long = "sort-by-length", help_heading = Some("FLAGS"))]
        length: bool,
        /// output reversed result
        #[arg(short = 'r', long = "reverse", help_heading = Some("FLAGS"))]
        reverse: bool,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        out: Option<String>,
    },
    /// search subsequences/motifs from fasta file
    search {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// specify uppercase pattern/motif, e.g., -p "ATC{2,}" or -p ATCCG
        ///search multiple pattern/motif, -p "ATCCG|GCTAA"
        #[arg(short = 'p', long = "pattern",verbatim_doc_comment)]
        pat: String,
        /// if specified, show header in result
        #[arg(short = 'H', long = "header", help_heading = Some("FLAGS"))]
        Header: bool,
        /// output search result file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str" )]
        output: Option<String>,
    },
    /// shuffle fasta sequences
    #[command(before_help = "note: all records will be readed into memory")]
    shuffle {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set rand seed.
        #[arg(short = 's', long = "seed", default_value_t = 69, value_name = "int")]
        seed: u64,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        out: Option<String>,
    },
    /// subsample sequences from big fasta file
    subfa {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set rand seed
        #[arg(short = 's', long = "seed", default_value_t = 69, value_name = "int")]
        seed: u64,
        /// reduce much memory but cost more time
        #[arg(short = 'r', long = "rdc", help_heading=Some("FLAGS"))]
        rdc: bool,
        /// subseq number
        #[arg(short = 'n', long = "num", value_name = "int")]
        num: usize,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    }, 
    /// split fasta file by sequence id
    split {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set output file extension, eg. fa, fa.gz, fna.xz, fna.bz2
        #[arg(short = 'e', long = "ext", value_name = "str")]
        ext: String,
        /// split fasta file output dir, default: current dir
        #[arg(short = 'o', long = "outdir", value_name = "str")]
        outdir: Option<String>,
    },
    /// simple summary for dna fasta files
    #[command(visible_alias = "stat")]
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
    /// show codon table and amino acid name
    codon {
        /// amino acid short name eg. S
        #[arg(short='n', long="name", value_name = "str")]
        name: Option<String>,
    }
}