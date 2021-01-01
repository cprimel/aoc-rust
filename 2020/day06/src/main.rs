use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn load_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() {
    let input = "input";
    let reader = load_input(input);

    let mut answers: HashMap<u8, i32> = HashMap::new();
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            sum_1 += answers.len();
            for (_key, val) in answers.iter() {
                if *val == counter {
                    sum_2 += 1;
                }
            }
            counter = 0;
            answers.clear();
        } else {
            counter += 1;
            for c in line.as_bytes() {
                let mut new_val = 1;
                if answers.contains_key(c) {
                    new_val += answers.get(c).unwrap();
                }
                answers.insert(*c, new_val);
            }
        }
    }
    sum_1 += answers.len();
    for (_key, val) in answers.iter() {
        if *val == counter {
            sum_2 += 1;
        }
    }
    println!("The sum of all affirmative responses is {}", sum_1);
    println!("The sum of questions to which individual groups' members had all affirmative responses is {}", sum_2);
}
