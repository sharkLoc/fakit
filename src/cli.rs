use clap::{Parser,value_parser};

#[derive(Parser, Debug)]
#[command(
    author = "sharkLoc",
    version = "0.2.10",
    about = "A simple program for fasta file manipulation",
    long_about = None,
    next_line_help = false,
    before_help = None,
    help_template = "{name}: {about}\n\nVersion: {version}\
    \nAuthors: {author} <mmtinfo@163.com>\
    \n\n{usage-heading} {usage}\n\n{all-args}\n"
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Subcli,
    /// set gzip compression level 1 (compress faster) - 9 (compress better) for gzip output file,
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
    /// Get first N records from fasta file
    topn {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// print first N fasta records
        #[arg(short = 'n', long = "num", default_value_t = 10)]
        num: usize,
        /// output fasta[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    }, 
    /// Convert fasta to fastq file
    fa2fq {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// fasta to fastq and generate fake fastq quality.
        #[arg(short = 'q', long = "qual", default_value_t = 'F')]
        qual: char,
        /// output fastq file name[.gz], or write to stdout, file name ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
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
    /// Strip of white spaces in fasta sequences
    flatten {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// output fasta file name[.gz], or write to stdout, file name ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// Print fasta records in a range
    range {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// skip first int fasta records
        #[arg(short = 's', long = "skip", default_value_t = 0)]
        skip: usize,
        /// take int fasta records
        #[arg(short = 't', long = "take")]
        take: usize,
        /// fasta output file name or write to stdout, files ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        out: Option<String>,

    },
    /// Re-length fasta sequence 
    relen {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// specify each seq length, 0 for a single line  
        #[arg(short = 'l', long = "len", default_value_t = 70)]
        len: usize,
        /// output fasta[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
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
        /// output fasta[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        output: Option<String>,
    },
    /// Get a reverse-complement of fasta file.
    #[command(visible_alias = "rev")]
    reverse {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// if set, just output reverse sequences
        #[arg(short = 'r', long = "reverse", help_heading = Some("FLAGS"))]
        rev: bool,
        /// output file name[.gz] or write to stdout, file ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
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
        /// output result[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
        ///header format: seqid    start   end gc_rate sequence
        #[arg(short = 'o', long = "out",verbatim_doc_comment )]
        output: Option<String>,
    },
    /// Sort fasta file by name/seq/gc/length
    #[command(before_help = "note: all records will be readed into memory")]
    sort {
        /// input fasta[.gz] file, or read from stdin
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
        /// output file name or write to stdout, file ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
    },
    /// Search subsequences/motifs from fasta file
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
        /// output search result[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out" )]
        output: Option<String>,
    },
    /// Shuffle fasta sequences
    #[command(before_help = "note: all records will be readed into memory")]
    shuffle {
        /// input fasta[.gz] file, or read from stdin
        input: Option<String>,
        /// set rand seed.
        #[arg(short = 's', long = "seed", default_value_t = 69)]
        seed: u64,
        /// output file name or write to stdout, file ending in .gz will be compressed automatically
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
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
        /// output fasta[.gz] file name, or write to stdout, file name ending in .gz will be compressed automatically
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