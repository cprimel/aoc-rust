use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{max, min};
use std::collections::HashSet;

fn load_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() {
    let filename = "input.txt";
    let reader = load_input(filename);

    let mut max_seat_id = 0;
    let mut min_seat_id = 128 * 8;
    let mut boarding_passes: HashSet<i32> = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let row_portion = &line[..line.len() - 3];

        // binary seat partition for row
        let mut row_bsp = 0b0;
        for (i, &c) in row_portion.as_bytes().iter().enumerate() {
            if c == b'B' {
                row_bsp += 0b1 << (6 - i);
            }
        }

        let mut column_bsp = 0b0;
        let column_portion = &line[line.len() - 3..];
        for (i, &c) in column_portion.as_bytes().iter().enumerate() {
            if c == b'R' {
                column_bsp += 0b1 << (2 - i);
            }
        }
        let seat_id = row_bsp * 8 + column_bsp;
        boarding_passes.insert(seat_id);
        max_seat_id = max(max_seat_id, seat_id);
        min_seat_id = min(min_seat_id, seat_id);
    }

    println!("Max seat ID is {}.", max_seat_id);
    println!("Min seat ID is {}.", min_seat_id);

    let mut my_seat_id:i32 = 0;
    for x in min_seat_id..max_seat_id {
        if !boarding_passes.contains(&x) {
            my_seat_id = x;
        }
    }

    println!("{}", my_seat_id);
}
