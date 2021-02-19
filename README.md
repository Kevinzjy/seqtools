# BIOLS-rs
Rust library for useful bioinformatic tools

# Usage

```bash
seqtools count tests/test.fa
seqtools filter -i tests/test.fa -l tests/test.txt -o tests/output.fa -m 10 -x 1000 -n
seqtools polya -i tests/test.fa -o tests/polya.txt -t 30 -k 10 -a 8
```