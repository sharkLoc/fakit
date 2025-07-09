use clap::{
    ArgAction, Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
    value_parser,
};
use std::path::PathBuf;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(styles = STYLES)]
#[command(
    name = "Fakit",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "A simple program for fasta file manipulation",
    long_about = None,
    next_line_help = false,
    disable_help_flag = true,
    disable_version_flag = true,
    propagate_version = true,
    before_help = r"Fakit supports reading and writing gzip (.gz) format.
Bzip2 (.bz2) and xz (.xz) format is supported since v0.3.0.
Under the same compression level, xz has the highest compression ratio but consumes more time.

Compression level:
  format   range   default   crate
  gzip     1-9     6         https://crates.io/crates/flate2
  bzip2    1-9     6         https://crates.io/crates/bzip2
  xz       1-9     6         https://crates.io/crates/xz2
  zstd     1-4     2         roughly equals to zstd 1, 3, 7, 11, respectively",
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
    #[arg(short = 'w', long = "line-width", default_value_t = 70, global = true, value_name = "int",
        help_heading = Some("Global Arguments")
    )]
    pub width: usize,

    /// set gzip/bzip2/xz compression level 1 (compress faster) - 9 (compress better) for gzip/bzip2/xz output file,
    /// just work with option -o/--out
    #[arg(long = "compress-level", default_value_t = 6, global = true, value_parser = value_parser!(u32).range(1..=9), value_name = "int",
        help_heading = Some("Global Arguments")
    )]
    pub compression_level: u32,

    /// if file name specified, write log message to this file, or write to stderr
    #[arg(long = "log", global = true, help_heading = Some("Global Arguments"), value_name = "str")]
    pub logfile: Option<String>,

    /// control verbosity of logging, [-v: Error, -vv: Warn, -vvv: Info, -vvvv: Debug, -vvvvv: Trace, defalut: Debug]
    #[arg(short = 'v', long = "verbosity", action = ArgAction::Count, global = true,
        default_value_t = 4, help_heading = Some("Global Arguments")
    )]
    pub verbose: u8,

    /// be quiet and do not show extra information
    #[arg(short = 'q', long = "quiet", global= true, help_heading = Some("Global FLAGS"))]
    pub quiet: bool,

    /// prints help information
    #[arg(short = 'h', long, action = ArgAction::Help, global= true, help_heading = Some("Global FLAGS"))]
    pub help: Option<String>,

    /// prints version information
    #[arg(short = 'V', long, action = ArgAction::Version, global= true, help_heading = Some("Global FLAGS"))]
    pub version: Option<String>,
}

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum Subcli {
    /// get first N records from fasta file
    #[command(visible_alias = "head")]
    topn {
        /// input fasta file, or read from stdin
        input: Option<PathBuf>,
        /// print first N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10, value_name = "int")]
        num: usize,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<PathBuf>,
    },
    /// get last N records from fasta file
    #[command(before_help = "note: the command need to read file twice, do not use in stream")]
    tail {
        /// input fasta file
        input: Option<String>,
        /// print last N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10, value_name = "int")]
        num: usize,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// convert fasta to fastq file
    fa2fq {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'Q', long = "qual", default_value_t = 'F', value_name = "char")]
        qual: char,
        /// output fastq file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// create index and random access to fasta files
    #[command(visible_alias = "fai")]
    faidx {
        /// input uncompressed fasta file
        input: Option<String>,
        /// fasta region format and start is 1-based, eg. chr1:1-5000 chr2:100-800
        /// usage:  fakit faidx seq.fa chr1:1-5000 chr2:100-800 ...
        #[arg(verbatim_doc_comment)]
        region: Option<Vec<String>>,
        /// output fasta file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
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
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// print fasta records in a range
    range {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// skip first int fasta records
        #[arg(short = 's', long = "skip", default_value_t = 0, value_name = "int")]
        skip: usize,
        /// take int fasta records
        #[arg(short = 't', long = "take", value_name = "int")]
        take: usize,
        /// fasta output file name or write to stdout, files ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
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
        #[arg(
            short = 'W',
            long = "window-size",
            default_value_t = 500,
            value_name = "int"
        )]
        wind: usize,
        /// set sliding window step size
        #[arg(
            short = 's',
            long = "step-size",
            default_value_t = 100,
            value_name = "int"
        )]
        step: usize,
        /// if specified, keep fasta format in output result
        #[arg(short = 'k', long = "keep", help_heading = Some("FLAGS"))]
        keep: bool,
        /// output result file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        ///header format: seqid    start   end gc_rate sequence
        #[arg(short = 'o', long = "out", verbatim_doc_comment, value_name = "str")]
        output: Option<String>,
    },
    /// grep fasta sequences by name/seq
    grep {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// specify regex pattern/motif, e.g., -p "ATC{2,}" or -p ATCCG, search multiple pattern/motif, -p "ATCCG|GCTAA"
        /// when searching by sequence name, the sequence prefix ">" is not included in the header.
        #[arg(
            short = 'p',
            long = "pattern",
            verbatim_doc_comment,
            value_name = "str"
        )]
        pat: String,
        /// grep sequences by full name
        #[arg(short = 'n', long = "by-name", help_heading = Some("FLAGS"))]
        name: bool,
        /// grep sequences by sequence
        #[arg(short = 's', long = "by-seq", help_heading = Some("FLAGS"))]
        seq: bool,
        /// grep sequences by sequence
        #[arg(short = 'i', long = "ignore-case", help_heading = Some("FLAGS"))]
        ignore: bool,
        /// output search result file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// convert all bases to lower/upper case, filter by length
    seq {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if specified, convert all bases to lowercase
        #[arg(short = 'l', long = "lower-case")]
        lower: bool,
        /// if specified, convert all bases to uppercase
        #[arg(short = 'u', long = "upper-case")]
        upper: bool,
        /// if specified, fasta sequences shorter than length required will be discarded
        #[arg(short = 'm', long = "min-len", value_name = "int")]
        min: Option<usize>,
        /// if specified, fasta sequences longer than length required will be discarded
        #[arg(short = 'M', long = "max-len", value_name = "int")]
        max: Option<usize>,
        /// if specified, fasta sequences gc content less than gc_min required will be discarded
        #[arg(short = 'g', long = "gc-min", value_name = "float")]
        gc_min: Option<f64>,
        /// if specified, fasta sequences gc content more than length required will be discarded
        #[arg(short = 'G', long = "gc-max", value_name = "float")]
        gc_max: Option<f64>,
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
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// specify uppercase pattern/motif, e.g., -p "ATC{2,}" or -p ATCCG
        ///search multiple pattern/motif, -p "ATCCG|GCTAA"
        #[arg(
            short = 'p',
            long = "pattern",
            verbatim_doc_comment,
            value_name = "str"
        )]
        pat: String,
        /// if specified, show header in result
        #[arg(short = 'H', long = "header", help_heading = Some("FLAGS"))]
        Header: bool,
        /// output search result file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// a simple kmer counter
    kmer {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set kmer size
        #[arg(
            short = 'k',
            long = "kmer-size",
            default_value_t = 21,
            value_name = "int"
        )]
        size: usize,
        /// add header info in output file
        #[arg(short = 'H', long, help_heading = Some("FLAGS"))]
        header: bool,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        out: Option<String>,
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
    /// report fasta sequence base count
    size {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// if specified, show more information
        #[arg(short='a', long="all", help_heading=Some("FLAGS"))]
        all: bool,
        /// output file name or write to stdout, file ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
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
    #[command(before_help = "note: each sequence results in a separate file")]
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
    /// split fasta file by sequence number
    split2 {
        /// input fasta file, or read from stdin
        input: Option<String>,
        /// set record number for each mini fasta file
        #[arg(short = 'n', long = "num", default_value_t = 100, value_name = "int")]
        num: usize,
        /// if specified, output gzip compressed file
        #[arg(short = 'z', long = "gzip", help_heading = Some("FLAGS"))]
        gzip: bool,
        /// if specified, output bzip2 compressed file
        #[arg(short = 'Z', long = "bzip2", help_heading = Some("FLAGS"))]
        bzip2: bool,
        /// if specified, output xz compressed file
        #[arg(short = 'x', long = "xz", help_heading = Some("FLAGS"))]
        xz: bool,
        /// set output mini fasta file prefix name
        #[arg(short = 'p', long = "prefix", default_value_t = String::from("sub"), value_name = "str")]
        name: String,
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
        /// output summary file name, or write to stdout, file name ending in .gz/.bz2/.xz will be compressed automatically
        #[arg(short = 'o', long = "out", value_name = "str")]
        output: Option<String>,
    },
    /// show codon table and amino acid name
    codon {
        /// amino acid short name eg. S
        #[arg(short = 'n', long = "name", value_name = "str")]
        name: Option<String>,
    },
}
