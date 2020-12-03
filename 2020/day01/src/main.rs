use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let data = read(File::open("input.txt")?)?;
    let mut map = HashMap::new();
    for element in data.iter() {
        map.insert(element, 0);
    }

    for elem in data.iter() {
        let key = 2020 - elem;
        if map.contains_key(&key) {
            println!("{} : {}", elem, key);
            println!("product: {}", elem * key);
            break;
        }
    }

    let mut found = false;
    for elem1 in data.iter() {
        if found { break; }
        let diff = 2020 - elem1;
        for elem2 in data.iter() {
            let key = diff - elem2;
            if map.contains_key(&key) {
                println!("{} : {} : {}", elem1, elem2, key);
                println!("product: {}", elem1 * elem2 * key);
                found = true;
                break;
            }
        }
    }

    Ok(())
}
