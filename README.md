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
fakit -h
fakit: a simple program for fasta file manipulation

Usage: fakit <COMMAND>

Commands:
  summ   summary fasta file
  stats  state fasta file
  fa2fq  convert fasta to fastq file
  faidx  crate index for fasta file
  fmt    format fasta file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## exeample
`test.fa`
```
>s1
GAGATCGGAGAAGATAGTTTTAGGGTTTGAGATTGAGAAGAAGATGAAGAAAATTTATGA
>s2
gactnacntacnncGCACAAACAGGACgatgatgttgatCCGTGTGTGTACGTGAGTTGG
>s3
GAGAGACTCTTCGTAAGACAGTAAGATTGTGAAAGTCA
```

<b>`fakit -u test.ta`</b>
```
>s1
GAGATCGGAGAAGATAGTTTTAGGGTTTGAGATTGAGAAGAAGATGAAGAAAATTTATGA
>s2
GACTNACNTACNNCGCACAAACAGGACGATGATGTTGATCCGTGTGTGTACGTGAGTTGG
>s3
GAGAGACTCTTCGTAAGACAGTAAGATTGTGAAAGTCA
```
<b>`fakit -u test.ta |fakit -w 30`</b>
```
>s1
GAGATCGGAGAAGATAGTTTTAGGGTTTGA
GATTGAGAAGAAGATGAAGAAAATTTATGA
>s2
GACTNACNTACNNCGCACAAACAGGACGAT
GATGTTGATCCGTGTGTGTACGTGAGTTGG
>s3
GAGAGACTCTTCGTAAGACAGTAAGATTGT
GAAAGTCA
```
<b>`fakit -u test.ta |fakit -w 0 |fakit -l`</b>
```
>s1
gagatcggagaagatagttttagggtttgagattgagaagaagatgaagaaaatttatga
>s2
gactnacntacnncgcacaaacaggacgatgatgttgatccgtgtgtgtacgtgagttgg
>s3
gagagactcttcgtaagacagtaagattgtgaaagtca
```
<b>`fakit -u test.ta |fakit -d 50`</b>
```
>s1
GAGATCGGAGAAGATAGTTTTAGGGTTTGAGATTGAGAAGAAGATGAAGAAAATTTATGA
>s2
GACTNACNTACNNCGCACAAACAGGACGATGATGTTGATCCGTGTGTGTACGTGAGTTGG
```
<b>`fakit -c r test.ta `</b>
```
>s1
AGTATTTAAAAGAAGTAGAAGAAGAGTTAGAGTTTGGGATTTTGATAGAAGAGGCTAGAG
>s2
GGTTGAGTGCATGTGTGTGCCtagttgtagtagCAGGACAAACACGcnncatncantcag
>s3
ACTGAAAGTGTTAGAATGACAGAATGCTTCTCAGAGAG
```
<b>`fakit -s test.ta `</b>
```
id      base_A  base_T  base_G  base_C  base_N  GC_Rate seq_Len
s1      24      16      19      1       0       0.33    60
s2      14      14      17      15      0       0.53    60
s3      14      9       10      5       0       0.39    38
```
<b>`fakit -f E test.ta `</b>
```
@s1
GAGATCGGAGAAGATAGTTTTAGGGTTTGAGATTGAGAAGAAGATGAAGAAAATTTATGA
+
EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
@s2
gactnacntacnncGCACAAACAGGACgatgatgttgatCCGTGTGTGTACGTGAGTTGG
+
EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
@s3
GAGAGACTCTTCGTAAGACAGTAAGATTGTGAAAGTCA
+
EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
```



<br>
** any bugs please report issues **💖
