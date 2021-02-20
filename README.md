# Seqtools

Utilities for the fasta/fastq format

## Synopsis

```bash
seqtools count s1.fa s2.fa s3.fq ...
seqtools filter -i tests/test.fa -l tests/test.txt -o tests/output.fa -m 10 -x 1000 -n
seqtools polya -i tests/test.fa -o tests/polya.txt -t 30 -k 10 -a 8
seqtools (-h | --help)
seqtools --version
```

## Commands 

### **count** - Summarize reads number and length distribution

```text
seqtools count s1.fa s2.fa s3.fq ...

Specify one or more space-seperated fasta/fastq file(s), and print the summary of each file, compressed gz/xz/bz format are also supported.

Example:
$ seqtools count sample01.fastq sample02.fq.gz

Output:
FileName,Reads,Bases,N01,N10,N50,N90,N99
sample01.fastq,21706,90724575,12482,8685,5562,2626,798
sample02.fq.gz,774555,585101975,2818,1591,813,476,304
```

### **filter** - filter reads according to reads id and sequence length

```text
seqtools filter [-l IdList] [-m MinLen] [-x MaxLen] [-n] -i in.fq -o out.fq

Options:
  -l FILE       ID list file (ID\nID)
  -x INT        Maximum sequence length to output [default: 0]
  -m INT        Minimum sequence length to output [default: 0]
  -n            Set for negative filtering instead of positive

  If -l option is provided, reads are then filtered according to the ids provided in the IdList file. By default, only reads included in the list will be kept for output. If -n option is provided, reads in the list will be dropped.

Example:
$ seqtools filter -i reads.fq -l seq_ids.txt -o out.fq -m 100 -x 1000 -n

Output:
Keeping 94025 sequence ids
After filtering:
Total n:        94025
Total bases:    34454727 bp
Min seq:        85 bp
Max seq:        2238 bp
```

### **polya** - scan reads for polyA sequence in 3'/5' terminal

```text
seqtools polya [options] -i FASTX -o TXT

Options:
  -k INT        Kmer size, smaller than 128 [default: 10]
  -a INT        Number of oligo(A/T) bases [default: 8]
  -t INT        Length of terminal region for scanning [default: 30]

  Reads will automically convert to capitalized sequence, and scan for kmers (-k) with more than (-a) A or Ts in the interminal region (-t). ID of reads that have polyA signals will be stored in output file (-o).

Example:
$ seqtools polya -i ./barcode01.fq.gz -o ./polyA_reads_id.txt

Output:
87.30% (82087/94025) reads have polyA tails!
```
