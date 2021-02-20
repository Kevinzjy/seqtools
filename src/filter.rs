use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

use failure::Error;
use needletail::parse_fastx_file;
use needletail::parser::LineEnding;
// TODO: Speed up writing

pub fn filter_by_ids(
    infile: &String, 
    outfile: &String,
    id_list: &String,
    flag_x: &String,
    flag_m: &String,
    flag_n: &bool,
) -> Result<(), Error> {
    // Load reads id
    let mut seq_map = BTreeMap::<String, usize>::new();
    let mut n_ids = 0;
    let min_len = flag_m.parse::<usize>().expect("Invalid -m paramter");
    let max_len = flag_x.parse::<usize>().expect("Invalid -x paramter");
    let is_neg = *flag_n;

    if id_list != "" {
        let id = Path::new(id_list);
        if id.is_file() {
            let id_file = File::open(id).expect("Failed to open id file");
            for line in BufReader::new(id_file).lines() {
                seq_map.insert(line.expect("Wrong line"), 1);
                n_ids += 1;
            }
            match is_neg {
                true => eprintln!("Dropping {} sequence ids", n_ids),
                false => eprintln!("Keeping {} sequence ids", n_ids)
            }
        } else {
            eprintln! ("{} does not exist or is a directory.", id_list);
            exit(1);
        }
    } else {
        println!("No id file specified, filtering reads length");
    }
    
    // Load input sequences
    let fastx = Path::new(infile);
    if !fastx.is_file() {
        eprintln! ("{} does not exist or is directory.", infile);
        exit(1);
    }
    let (mut n_reads, mut n_bases, mut min_seq, mut max_seq) = (0, 0, 0, 0);
    let mut reader = parse_fastx_file(infile).expect("Invalid path/file");  
    let mut output = File::create(outfile).expect("Unable to create output file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("Invalid record");

        // filter sequence id
        let seq_id = std::str::from_utf8(seqrec.id()).expect("Unable to get sequence id").to_string();
        let mut is_filter = false;
        for x in seq_id.split(" ") {
            match seq_map.get(x) {
               Some(_ret) => is_filter = true,
               None => continue
            }
        }
        if id_list != "" && (is_filter ^ !is_neg) {
            continue;
        }

        // filter length
        let seq_len = seqrec.num_bases();
        if max_len != 0 && seq_len > max_len as usize {
            continue;
        }
        if min_len != 0 && seq_len < min_len as usize {
            continue;
        }
        if min_seq == 0 || seq_len < min_seq {
            min_seq = seq_len;
        }
        if seq_len > max_seq {
            max_seq = seq_len;
        }
        n_reads += 1;
        n_bases += seq_len;
        seqrec.write(&mut output, Some(LineEnding::Unix)).expect("Failed to output");
    }

    eprintln!("After filtering:");
    eprintln!("Total n:\t{}", n_reads);
    eprintln!("Total bases:\t{} bp", n_bases);
    eprintln!("Min seq:\t{} bp", min_seq);
    eprintln!("Max seq:\t{} bp", max_seq);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let flag_i = String::from("tests/test.fa");
        let flag_o = String::from("tests/output.fa");
        let flag_l = String::from("tests/test.txt");
        let flag_m = String::from("10");
        let flag_x = String::from("10000");
        let flag_n = false;

        filter_by_ids(&flag_i, &flag_o, &flag_l, &flag_x, &flag_m, &flag_n).expect("Failed in seqfilter");
    }
}