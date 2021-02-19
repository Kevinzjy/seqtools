use std::path::Path;
use failure::Error;
use needletail::parse_fastx_file;

pub fn fast_count(filename: &String) -> Result<(), Error> {
    let f = Path::new(filename);
    if !f.is_file() {
        eprintln! ("{} does not exist or is directory.", filename);
        return Ok(());
    }

    let mut n_reads = 0;
    let mut n_bases = 0;
    let mut seq_lens: Vec<usize> = vec![];
    seq_lens.reserve(10_000_000);

    if let Ok(_) = parse_fastx_file(&filename) {
        // Pass
    } else {
        eprintln! ("{} is not a valid fasta/fastq record.", filename);
        return Ok(());
    }

    let mut reader = parse_fastx_file(&filename).expect("Invalid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let seq_len = seqrec.num_bases();
        n_reads += 1;
        n_bases += seq_len;
        seq_lens.push(seq_len);
    }
    seq_lens.sort();
    seq_lens.reverse();

    let target_01 = n_bases / 100;
    let target_10 = n_bases / 10;
    let target_50 = n_bases / 2;
    let target_90 = n_bases * 9 / 10;
    let target_99 = n_bases * 99 / 100;
    let mut cum_len = 0;
    let (mut n01, mut n10, mut n50, mut n90, mut n99) = (0, 0, 0, 0, 0);
    let (mut set_01, mut set_10, mut set_50, mut set_90, mut set_99) = (false, false, false, false, false);

    for i in seq_lens.iter() {
        cum_len += i;
        if cum_len >= target_01 && !set_01 {
            n01 = *i;
            set_01 = true;
        }
        if cum_len >= target_10 && !set_10 {
            n10 = *i;
            set_10 = true;
        }
        if cum_len >= target_50 && !set_50 {
            n50 = *i;
            set_50 = true;
        }
        if cum_len >= target_90 && !set_90 {
            n90 = *i;
            set_90 = true;
        }
        if cum_len >= target_99 && !set_99 {
            n99 = *i;
            set_99 = true;
        }
        if set_01 && set_10 && set_50 && set_90 && set_99 {
            break;
        }
    }

    let ret = vec![filename.to_string(), n_reads.to_string(), n_bases.to_string(), n01.to_string(), n10.to_string(), n50.to_string(), n90.to_string(), n99.to_string()];
    let joined = ret.join(",");
    println!("{}", joined);
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let test_file: String = String::from("tests/test.fa");
        fast_count(&test_file).expect("Failed in fast count");
    }
}