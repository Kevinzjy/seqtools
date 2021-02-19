use std::collections::{HashMap, BTreeMap};

use needletail::{parse_fastx_file, Sequence, FastxReader};

use biols::utils;

// const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
// const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const USAGE: &'static str = "
// Pseudo-Alignment of Transcriptome Reads from Oxford Nanopore Sequencing

// Usage:
//   patron-rs [--num-threads=<n>] -r FASTA <reads>
//   patron-rs --help | --version

// Options:
//     -n --num-threads N  Number of worker theads [default: 2]
//     -r FASTA            Reference fasta
//     -h --help           Show this screen
//     -v --version        Show version
// ";

fn main() {
    let trim_len = 30;
    let length_threshold = 300;

    let mut n_reads = 0;
    let mut n_bases = 0;
    let mut polya_reads = 0;
    let mut polya_bases = 0;

    let filename = "tests/test.fq.gz";
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        let seq = seqrec.sequence();
        let seq_len = seq.len();
        if seq_len < length_threshold {
            continue;
        }
        let head = &seq[0..trim_len];
        let tail = &seq[seq_len-trim_len..seq_len];

        n_bases += seqrec.num_bases();
        n_reads += 1;

        let is_head = scan_seq(head);
        let is_tail = scan_seq(tail);

        if is_head | is_tail {
            polya_reads += 1;
            polya_bases += seqrec.num_bases();
        }
    }
    println!("{},{},{},{}", n_reads, n_bases, polya_reads, polya_bases);
}

fn scan_seq(seq: &[u8]) -> bool {
    let norm_seq = seq.normalize(false); // remove ambiguous bases and newlines
    let rc = norm_seq.reverse_complement(); //make reverse complemented copy
    let mut is_valid = false;
    for (_, kmer, _) in norm_seq.canonical_kmers(10, &rc) {
        let map = counter(kmer);
        let is_polya = detect_polya(map);
        if is_polya {
            let kstr = std::str::from_utf8(kmer).unwrap().to_string();
            // println! ("Found polyA: {}", kstr);
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

fn detect_polya(map: HashMap::<u8,u8>) -> bool {
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
    max_n >= 8
}

// #[derive(Clone, Debug, Deserialize)]
// struct Args {
//     arg_reads: String,

//     flag_r: String,
//     flag_num_threads: usize,

//     flag_version: bool,
//     flag_help: bool,
// }

// fn main() -> Result<(), Error> {
//     // Parse parameters
//     let args: Args = Docopt::new(USAGE)
//         .and_then(|d| d.deserialize())
//         .unwrap_or_else(|e| e.exit());

//     if args.flag_version {
//         print! ("{} {}", PKG_NAME, PKG_VERSION);
//         return Ok(());
//     }

//     // Init logger
//     utils::info("Start running PATRON");

//     // Generate index from reference fasta
//     let reference = Path::new(&args.flag_r);
//     match reference.exists() {
//         true => utils::info("Loading fastq reads"),
//         false => utils::error(format!("Can not find file: {}", &args.flag_r)),
//     }

//     build_index::read_transcripts(&reference);

//     utils::info("Finished!");

//     Ok(())
// }
