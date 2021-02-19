use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use failure::Error;
use needletail::{parse_fastx_file, Sequence};
use std::io::{Write, BufWriter};

pub fn scan_polya_reads(
    infile: &String,
    outfile: &String,
    flag_t: &String,
    flag_k: &String,
    flag_a: &String
) -> Result<(), Error> {
    let f = Path::new(infile);
    if !f.is_file() {
        eprintln! ("{} does not exist or is directory.", infile);
        return Ok(());
    }

    let mut n_reads = 0;
    let mut a_reads = 0;

    let l = flag_t.parse::<usize>().expect("Invalid -m paramter");
    let k = flag_k.parse::<u8>().expect("Invalid -m paramter");
    let n = flag_a.parse::<u8>().expect("Invalid");

    let mut reader = parse_fastx_file(&infile).expect("Invalid path/file");
    let mut output = BufWriter::new(File::create(outfile).expect("Unable to create file"));
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let seq = seqrec.sequence();
        let seq_len = seqrec.num_bases();
        if seq_len < l {
            continue
        }
        let head = &seq[0..l];
        let tail = &seq[seq_len-l..seq_len];

        let is_head = scan_seq(head, k, n);
        let is_tail = scan_seq(tail, k, n);

        n_reads += 1;
        if is_head | is_tail {
            let seq_id = std::str::from_utf8(seqrec.id()).expect("Unable to get sequence id").to_string();
            let vec: Vec<&str> = seq_id.split(" ").collect();
            write!(output, "{0}\n", vec[0]).expect("Write Error");
            a_reads += 1;
        }
    }
    eprintln!("{:.2}% ({}/{}) reads have polyA tails!", 100.0 * a_reads as f64 / n_reads as f64, a_reads, n_reads);
    Ok(())
}

fn scan_seq(seq: &[u8], kmer: u8, oligo: u8) -> bool {
    let norm_seq = seq.normalize(false); // remove ambiguous bases and newlines
    let rc = norm_seq.reverse_complement(); //make reverse complemented copy
    let mut is_valid = false;
    for (_, kmer, _) in norm_seq.canonical_kmers(kmer, &rc) {
        let map = counter(kmer);
        let is_polya = detect_polya(map, oligo);
        if is_polya {
            is_valid = true;
            break;
        }
    }
    is_valid
}

fn counter(seq: &[u8]) -> HashMap::<u8,u8> {
    let mut map = HashMap::<u8,u8>::new();
    let len = seq.len();
    for i in 0..len {
        let key_exists = map.contains_key(&seq[i]);
        if key_exists {
            let val = map.get_mut(&seq[i]).unwrap();
            *val += 1;
        } else {
            map.entry(seq[i]).or_insert(1);
        }
    }
    map
}

fn detect_polya(map: HashMap::<u8,u8>, oligo: u8) -> bool {
    let alphabets = [65, 84]; // detect oligo-dA or oligo-dT
    let mut max_n = 0;
    for i in alphabets.iter() {
        let key_exists = map.contains_key(&i);
        if key_exists {
            let val = map.get(&i).unwrap();
            if *val > max_n {
                max_n = *val;
            }
        }
    }
    max_n >= oligo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let infile = String::from("tests/test.fa");
        let outfile = String::from("tests/polya.txt");

        let t = String::from("10");
        let k = String::from("10");
        let a = String::from("8");
        scan_polya_reads(&infile, &outfile, &t, &k, &a).expect("Failed");
    }
}