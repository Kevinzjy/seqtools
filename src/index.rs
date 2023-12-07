use std::fs::File;
use std::path::Path;
use failure::Error;
use needletail::parse_fastx_file;
use std::io::{Write, BufWriter};

pub fn fast_index(filename: &String) -> Result<(), Error> {
    let f = Path::new(filename);
    if !f.is_file() {
        eprintln! ("{} does not exist or is directory.", filename);
        return Ok(());
    }

    if let Ok(_) = parse_fastx_file(&filename) {
        // Pass
    } else {
        eprintln! ("{} is not a valid fasta/fastq record.", filename);
        return Ok(());
    }

    let outfile = [filename, ".idx"].join("");
    let mut output = BufWriter::new(File::create(outfile).expect("Unable to create file"));
    let mut reader = parse_fastx_file(&filename).expect("Invalid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let seq_id = std::str::from_utf8(seqrec.id()).expect("Unable to get sequence id").to_string();
        let vec: Vec<&str> = seq_id.split(" ").collect();
        let seq_len = seqrec.num_bases();
        write!(output, "{0}\t{1}\n", vec[0], seq_len).expect("Write Error");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let infile = String::from("tests/test.fa");
        fast_index(&infile).expect("Failed");
    }
}