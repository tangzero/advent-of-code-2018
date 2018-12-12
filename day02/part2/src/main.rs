use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn found_similar(id: &String, others: &Vec<String>) -> Option<String> {
    for other in others {
        if id == other {
            continue;
        }

        for i in 0..id.len() {
            let mut new_id = id.clone();
            new_id.remove(i);

            let mut new_other = other.clone();
            new_other.remove(i);

            if new_id == new_other {
                return Some(new_id);
            }
        }
    }
    None
}

fn main() {
    let filename = env::args().nth(1).expect("a filename");
    let file = File::open(filename).expect("a input file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("a line"))
        .collect();

    for id in &lines {
        match found_similar(id, &lines) {
            Some(id) => {
                println!("{}", id);
                break;
            }
            None => (),
        }
    }
}
