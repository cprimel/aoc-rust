use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut tree_count = 0;
    let mut horizontal_pos = 1;

    let horizontal_move = 3;

    for line in reader.lines() {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        let len_segment = bytes.len();

        if horizontal_pos > len_segment {
            horizontal_pos = horizontal_pos - len_segment;
        }

        if bytes[horizontal_pos - 1] == b'#' {
            tree_count += 1;
        }
        horizontal_pos += horizontal_move;
    }
    println!("{}", tree_count);
}
