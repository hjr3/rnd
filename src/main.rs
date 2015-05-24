///
/// rnd - A program that accepts new-line delimited text over stdin and
/// randomly chooses one of the items or shuffles the entire list.
///
/// # Examples
///
/// Random choice from input:
///
/// ```
/// echo "one
/// two
/// three" | target/debug/rnd
/// two
/// ```
///
/// Shuffle entire list:
///
/// ```
/// echo "one
/// two
/// three" | target/debug/rnd -s
/// two
/// one
/// three
/// ```
///

extern crate getopts;
extern crate rand;

use getopts::Options;
use rand::{thread_rng, Rng};
use std::env;
use std::io::stdin;

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help menu");
    opts.optflag("s", "shuffle", "Shuffle input");
    let matches = opts.parse(&args[1..]).ok().expect("Unable to parse options");

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    let mut input = Vec::new();
    let mut stdin = stdin();
    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Err(e) => panic!(e.to_string()),
            Ok(_) => {
                input.push(line)
            }
        }
    }

    if input.is_empty() {
        return;
    }

    let mut rng = thread_rng();

    if matches.opt_present("s") {
        rng.shuffle(&mut input);
        for i in input {
            print!("{}", i);
        }
    } else {
        let choice = rng.choose(&input).expect("Failed to choose from input");
        print!("{}", choice);
    }
}
