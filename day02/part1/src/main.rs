use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn any_letter_appears_n_times(id: &String, times: usize) -> bool {
    for c in id.chars() {
        if id.replace(c, "").len() == (id.len() - times) {
            return true;
        }
    }
    false
}

fn any_letter_appears_two_times(id: &String) -> bool {
    any_letter_appears_n_times(id, 2)
}

fn any_letter_appears_three_times(id: &String) -> bool {
    any_letter_appears_n_times(id, 3)
}

fn main() {
    let filename = env::args().nth(1).expect("a filename");
    let file = File::open(filename).expect("a input file");

    let mut two_count = 0;
    let mut three_count = 0;

    for line in BufReader::new(file).lines() {
        let id = line.expect("a valid input");

        if any_letter_appears_two_times(&id) {
            two_count += 1;
        }

        if any_letter_appears_three_times(&id) {
            three_count += 1;
        }
    }

    let checksum = two_count * three_count;
    println!("{}", checksum);
}
