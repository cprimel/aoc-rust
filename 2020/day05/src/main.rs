use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

fn load_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() {
    let filename = "input.txt";
    let reader = load_input(filename);

    let mut max_seat_id = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let row_portion = &line[..line.len() - 3];

        // binary seat partition for row
        let mut row_bsp = 0b0;
        for (i, &c) in row_portion.as_bytes().iter().enumerate() {
            if c == b'B' {
                row_bsp += 0b1 << (6-i);
            }
        }

        let mut column_bsp = 0b0;
        let column_portion = &line[line.len() - 3..];
        for (i, &c) in column_portion.as_bytes().iter().enumerate() {
            if c == b'R' {
                column_bsp += 0b1 << (2-i);
            }
        }
        let seat_id = row_bsp * 8 + column_bsp;
        max_seat_id = max(max_seat_id, seat_id);
    }
    
    println!("Max seat ID is {}.", max_seat_id);
}
