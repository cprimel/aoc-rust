use std::fs::File;
use std::io::{copy,BufRead, BufReader};

fn count_trees_on_path(horizontal_move: usize, vertical_shift:usize, slope: &[u8]) -> i32 {
    let mut tree_count = 0;
    let mut horizontal_pos = 1;
    let mut vertical_counter = vertical_shift;

    for row in slope.lines() {
        if vertical_counter == vertical_shift {
            let row = row.unwrap();
            let row_bytes = row.as_bytes();
            let row_size = row_bytes.len();

            if horizontal_pos > row_size {
                horizontal_pos -= row_size;
            }

            if row_bytes[horizontal_pos - 1] == b'#' {
                tree_count += 1;
            }
            horizontal_pos += horizontal_move;
            vertical_counter = 1;
        } else {
            vertical_counter += 1;
        }


    }

    tree_count
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut writer: Vec<u8> = vec![];
    let result = copy(&mut reader, &mut writer);

    if result.is_err() {
        println!{"Error: {:?}",result };
    }

    let mut result: i32 = 1;


    println!("{}", count_trees_on_path(3,1,&writer));

    result *= count_trees_on_path(1,1,&writer);
    result *= count_trees_on_path(3,1,&writer);
    result *= count_trees_on_path(5,1,&writer);
    result *= count_trees_on_path(7,1,&writer);
    result *= count_trees_on_path(1,2,&writer);

    println!("{}", result);
}
