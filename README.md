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
fakit --help
fakit: a simple program for fasta file manipulation

Usage: fakit <COMMAND>

Commands:
  topn
          get first N records from fasta file
  fa2fq
          convert fasta to fastq file
  faidx
          crate index and random access to fasta files
  relen
          re-length fasta sequence
  rename
          rename sequence id in fasta file
  window
          stat dna fasta gc content by sliding windows
  search
          search subsequences/motifs from fasta file
  subfa
          subsample sequences from big fasta file
  split
          split fasta file by sequence id
  summ
          a simple summary for DNA fasta files
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help
  -V, --version
          Print version

```

<br>
** any bugs please report issues **💖
