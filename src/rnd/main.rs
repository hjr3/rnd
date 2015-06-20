///
/// rnd - A program that accepts new-line delimited text over stdin and
/// randomly chooses one of the items.
///
/// # Example
///
/// ```
/// echo "one
/// two
/// three" | target/debug/rnd
/// two
/// ```
///

extern crate rand;

use rand::{thread_rng, Rng};
use std::io::stdin;

fn main() {

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

    let choice = rng.choose(&input).expect("Failed to choose from input");
    print!("{}", choice);
}
