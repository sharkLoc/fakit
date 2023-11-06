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
          Get first N records from fasta file
  fa2fq
          Convert fasta to fastq file
  faidx
          Create index and random access to fasta files
  relen
          Re-length fasta sequence
  rename
          Rename sequence id in fasta file
  window
          Stat dna fasta gc content by sliding windows
  search
          Search subsequences/motifs from fasta file
  subfa
          Subsample sequences from big fasta file
  split
          Split fasta file by sequence id
  summ
          A simple summary for DNA fasta files
  codon
          Show codon table
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
