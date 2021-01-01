use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn load_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() {
    let input = "input";
    let reader = load_input(input);

    let mut answers: HashSet<u8> = HashSet::new();
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            sum += answers.len();
            answers.clear();
        } else {
            for &c in line.as_bytes() {
                answers.insert(c);
            }
        }
    }
    sum += answers.len();
    println!("The sum of affirmative responses is {}", sum);
}
