use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::collections::HashSet;

fn main() {
    let filename = env::args().nth(1).expect("a filename");
    let mut file = File::open(filename).expect("a input file");

    let mut acc: i64 = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(acc);

    loop {
        for line in BufReader::new(&file).lines() {
            let value: i64 = line.expect("a valid input").parse().expect("a number");
            acc += value;

            if frequencies.contains(&acc) {
                println!("{}", acc);
                return;
            }

            frequencies.insert(acc);
        }
        file.seek(SeekFrom::Start(0)).expect("should reset the file");
    }
}
