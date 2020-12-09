use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if is_valid_password(&*line) {
            count += 1;
        }
    }
    println!("# of valid passwords: {}", count);
}

fn is_valid_password(line: &str) -> bool {

    let index = line.find('-').unwrap();
    let min: i32 = line[0..index].parse().unwrap();

    let next_index = line.find(' ').unwrap();
    let max:i32 = line[(index+1)..next_index].parse().unwrap();

    let char: char = line[(next_index+1)..(next_index+2)].parse().unwrap();

    let password: String = line[(next_index+2)..].parse().unwrap();

    let mut char_counter = 0;
    for c in password.chars() {
        if c == char {
            char_counter += 1;
        }
    }
    (char_counter >= min) && (char_counter <= max)
}

