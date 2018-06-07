#[macro_use]
extern crate serde_derive;
extern crate docopt;

extern crate conio;

use std::process::exit;

use docopt::Docopt;

use conio::cmd;

const USAGE: &'static str = "
Concurrent I/O microbenchmark.

Usage:
  conio init <file> <num-segments>
  conio read [options] <file>
  conio write [options] <file>
  conio (-h | --help)

Arguments:
  file          File path to issues I/Os.
  num_segments  Number of segments of the file to create.

Options:
  -h --help   Show this screen.
  --direct    Use O_DIRECT (only on Linux).
  -n THREADS  I/O concurrency [default: 1].
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: String,
    arg_num_segments: u32,
    flag_direct: bool,
    flag_n: u8,
    cmd_init: bool,
    cmd_read: bool,
    cmd_write: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_init {
        println!("Initializing {} ...", &args.arg_file);

        cmd::init(&args.arg_file, args.arg_num_segments).unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(1);
        });
    } else if args.cmd_read {
        println!("Reading {} ...", &args.arg_file);

        let (elapsed, sum) = cmd::read(args.arg_file, args.flag_direct, args.flag_n)
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            });
        println!("{:?}\t{}", elapsed, sum);
    } else if args.cmd_write {
        println!("Writing {} ...", &args.arg_file);

        let elapsed =
            cmd::write(args.arg_file, args.flag_direct, args.flag_n).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            });
        println!("{:?}", elapsed);
    }
}
