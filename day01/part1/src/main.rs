use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("a filename");
    let file = File::open(filename).expect("a input file");

    let mut acc: i64 = 0;

    for line in BufReader::new(file).lines() {
        let value: i64 = line.expect("a valid input").parse().expect("a number");
        acc += value;
    }

    println!("{}", acc);
}
