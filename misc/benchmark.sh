#!/bin/bash
# RUSTFLAGS=-g cargo build --release
cargo build --release
# time ./target/release/seqtools count tests/test.fastq.gz
# time ~/git/BIOLS/bin/fast_count tests/test.fastq.gz
# time ./target/release/seqtools filter -i tests/test.fastq.gz -l tests/read_ids.txt -o tests/output1.fastq -m 10 -x 1000 -n
# time ~/git/BIOLS/bin/seqfilter -i tests/test.fastq.gz -l tests/read_ids.txt -o tests/output2.fastq -m 10 -x 1000 -n 
time ./target/release/seqtools polya -i tests/test.fastq.gz -o tests/polya.txt -t 30 -k 10 -a 8