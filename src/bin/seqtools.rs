use docopt::Docopt;
use serde::Deserialize;
use failure::Error;

use seqtools::{count, filter, scan};

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Seqtools (Tools for manipulate fasta/fastq files)

Usage:
  seqtools count <reads>...
  seqtools filter [options] -i FASTX -o FASTX
  seqtools polya [options] -i FASTX -o TXT
  seqtools (-h | --help)
  seqtools --version

Options:
  -i FILE       Input file (fasta/fastq)
  -o FILE       Output file
  -h --help     Show this screen
  --version     Show version

Options(Filter):
  -l FILE       ID list file (one ID per line)
  -x INT        Maximum sequence length to output [default: 0]
  -m INT        Minimum sequence length to output [default: 0]
  -n            Set for negative filtering instead of positive

Options(polya scanning):
  -k INT        Kmer size, smaller than 128 [default: 10]
  -a INT        Number of oligo(A/T) bases [default: 8]
  -t INT        Length of terminal region for scanning [default: 30]
";

#[derive(Clone, Debug, Deserialize)]
struct Args {
    // fast_count
    cmd_count: bool,
    arg_reads: Vec<String>,
    // seqfilter
    cmd_filter: bool,
    flag_i: String,
    flag_o: String,
    flag_l: String,
    flag_x: String,
    flag_m: String,
    flag_n: bool,
    // polyA scanner
    cmd_polya: bool,
    flag_k: String,
    flag_a: String,
    flag_t: String,
    // generic options
    flag_version: bool,
    flag_help: bool,
}

fn main() -> Result<(), Error> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        print! ("{}, version:{}", PKG_NAME, PKG_VERSION);
        return Ok(());
    }

    if args.cmd_count {
        println!("FileName,Reads,Bases,N01,N10,N50,N90,N99");
        for fname in args.arg_reads.iter() {
            let _ = count::fast_count(fname);
        }
        return Ok(());
    }

    if args.cmd_filter {
        let _ = filter::filter_by_ids(
            &args.flag_i,
            &args.flag_o,
            &args.flag_l,
            &args.flag_x,
            &args.flag_m,
            &args.flag_n,
        );
        return Ok(());
    }

    if args.cmd_polya {
        let _ = scan::scan_polya_reads(
            &args.flag_i,
            &args.flag_o,
            &args.flag_t,
            &args.flag_k,
            &args.flag_a,
        );
        return Ok(());
    }

    Ok(())
}
