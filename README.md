# fakit
🦀 a simple program for fasta file manipulation 

## install

```bash
cargo install fakit

# or 

git clone https://github.com/sharkLoc/fakit.git
cd fakit
cargo b --release
# mv target/release/fakit to anywhere you want 
```





## usage

```bash
fakit: A simple program for fasta file manipulation

Version: 0.2.9
Authors: sharkLoc <mmtinfo@163.com>

Usage: fakit [OPTIONS] <COMMAND>

Commands:
  topn    Get first N records from fasta file
  fa2fq   Convert fasta to fastq file
  faidx   Create index and random access to fasta files [aliases: fai]
  relen   Re-length fasta sequence
  rename  Rename sequence id in fasta file
  window  Stat dna fasta gc content by sliding windows
  search  Search subsequences/motifs from fasta file
  subfa   Subsample sequences from big fasta file
  split   Split fasta file by sequence id
  summ    A simple summary for DNA fasta files
  codon   Show codon table and amino acid name
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Global FLAGS:
  -q, --quiet  be quiet and do not show extra information
```

<br>
** any bugs please report issues **💖
