use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count_1 = 0;
    let mut count_2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if is_valid_password_part1(&*line) {
            count_1 += 1;
        }
        if is_valid_password_part2(&*line) {
            println!("{}", line);
            count_2 += 1;
        }
    }
    println!("# of valid passwords for part 1: {}", count_1);
    println!("# of valid passwords for part 2: {}", count_2);
}

fn is_valid_password_part1(line: &str) -> bool {
    let index = line.find('-').unwrap();
    let min: i32 = line[0..index].parse().unwrap();

    let next_index = line.find(' ').unwrap();
    let max: i32 = line[(index + 1)..next_index].parse().unwrap();

    let char: char = line[(next_index + 1)..(next_index + 2)].parse().unwrap();

    let password: String = line[(next_index + 2)..].parse().unwrap();

    let mut char_counter = 0;
    for c in password.chars() {
        if c == char {
            char_counter += 1;
        }
    }
    (char_counter >= min) && (char_counter <= max)
}

fn is_valid_password_part2(line: &str) -> bool {
    let index = line.find('-').unwrap();
    let mut position_1: usize = line[0..index].parse().unwrap();
    position_1 += 1;

    let next_index = line.find(' ').unwrap();
    let mut position_2: usize = line[(index + 1)..next_index].parse().unwrap();
    position_2 += 1;


    let char: char = line[(next_index + 1)..(next_index + 2)].parse().unwrap();

    let password: String = line[(next_index + 2)..].parse().unwrap();

    (password.as_bytes()[position_1] as char == char && password.as_bytes()[position_2] as char != char) || (password.as_bytes()[position_1] as char != char && password.as_bytes()[position_2] as char == char)
}