# fakit
🦀 a simple program for fasta file manipulation 

## install

```bash
git clone https://github.com/sharkLoc/fakit.git
cd fakit
cargo b --release
# mv target/release/fakit to anywhere you want 
```

## usage

```bash
fakit -h
fqkit: a simple program for fasta file manipulation

Usage: fakit [OPTIONS] [INPUT]

Arguments:
  [INPUT]  input fasta[.gz] file

Options:
  -u, --upper           convert base to uppercase
  -l, --lower           convert base to lowercase
  -w, --length <LEN>    base number of each line, 0 for long single line
  -f, --fake <FAKE>     fasta to fastq and generate fake fastq quality
  -d, --drop <DROP>     drop sequences with length shorter than int
  -c, --convert <CONV>  r for reverse seq, m for match seq
  -s, --summary         simple statistics of fasta file
  -h, --help            Print help information
  -V, --version         Print version information
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
