# fakit
🦀 a simple program for fasta file manipulation 

## install latest version

```bash
cargo install --git https://github.com/sharkLoc/fakit.git
```

## install

```bash
cargo install fakit
```


## usage

```bash
fakit: A simple program for fasta file manipulation

Version: 0.2.10
Authors: sharkLoc <mmtinfo@163.com>

Usage: fakit [OPTIONS] <COMMAND>

Commands:
  topn     Get first N records from fasta file
  fa2fq    Convert fasta to fastq file
  faidx    Create index and random access to fasta files [aliases: fai]
  flatten  Strip of white spaces in fasta sequences
  range    Print fasta records in a range
  relen    Re-length fasta sequence
  rename   Rename sequence id in fasta file
  reverse  Get a reverse-complement of fasta file [aliases: rev]
  window   Stat dna fasta gc content by sliding windows
  sort     Sort fasta file by name/seq/gc/length
  search   Search subsequences/motifs from fasta file
  shuffle  Shuffle fasta sequences
  subfa    Subsample sequences from big fasta file
  split    Split fasta file by sequence id
  summ     A simple summary for DNA fasta files
  codon    Show codon table and amino acid name
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Global Arguments:
      --compress-level <int>  set gzip compression level 1 (compress faster) - 9 (compress better) for gzip output file, just work with option -o/--out [default: 6]
      --log <str>             if file name specified, write log message to this file, or write to stderr
  -v, --verbosity <str>       control verbosity of logging, possible values: {error, warn, info, debug, trace} [default: debug]

Global FLAGS:
  -q, --quiet  be quiet and do not show extra information
```

<br>
** any bugs please report issues **💖
