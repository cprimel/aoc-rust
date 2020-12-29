use std::fs::File;
use std::io::{BufRead, BufReader};

struct Passport {
    passport_id: String,
    birth_year: String,
    issued_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
}

impl Passport {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            passport_id: String::with_capacity(10),
            birth_year: String::with_capacity(5),
            issued_year: String::with_capacity(5),
            expiration_year: String::with_capacity(5),
            height: String::with_capacity(10),
            hair_color: String::with_capacity(10),
            eye_color: String::with_capacity(5),
        }
    }
}

fn contains_required_fields(passport: &Passport) -> bool {
    !passport.birth_year.is_empty()
        && !passport.issued_year.is_empty()
        && !passport.expiration_year.is_empty()
        && !passport.height.is_empty()
        && !passport.hair_color.is_empty()
        && !passport.eye_color.is_empty()
        && !passport.passport_id.is_empty()
}

fn validate_range(value: i32, min: i32, max: i32) -> bool {
    value >= min && value <= max
}

fn validate_year_range(value_as_string: &str, min: i32, max: i32) -> bool {
    if value_as_string.is_empty() {
        false
    } else {
        let year = value_as_string.parse::<i32>().unwrap();
        validate_range(year, min, max)
    }
}

fn validate_birth_year(birth_year: &str) -> bool {
    validate_year_range(birth_year, 1920, 2002)
}

fn validate_issued_year(issued_year: &str) -> bool {
    validate_year_range(issued_year, 2010, 2020)
}

fn validate_expiration_year(expiration_year: &str) -> bool {
    validate_year_range(expiration_year, 2020, 2030)
}

fn validate_height(height_field: &str) -> bool {
    if height_field.is_empty() {
        false
    } else {
        let field_len = height_field.len();
        let height_unit = &height_field[field_len - 2..];

        if height_unit == "in" {
            let height_val = height_field[..field_len - 2].parse::<i32>().unwrap();
            validate_range(height_val, 59, 76)
        } else if height_unit == "cm" {
            let height_val = height_field[..field_len - 2].parse::<i32>().unwrap();
            validate_range(height_val, 150, 193)
        } else {
            false
        }
    }
}

fn validate_hair_color(hair_field: &str) -> bool {
    if hair_field.is_empty() {
        false
    } else if &hair_field[..1] == "#" && hair_field.len() == 7 {
        for &c in hair_field[1..].as_bytes() {
            if (c >= 97 && c <= 102) || (c >= 48 && c <= 57) {} else { return false; }
        }
        true
    } else {
        false
    }
}

fn validate_eye_color(eye_color: &str) -> bool {
    if !eye_color.is_empty() {
        matches!(eye_color,"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    } else { false }
}

fn validate_passport_id(passport_id: &str) -> bool {
    if !passport_id.is_empty() && passport_id.len() == 9 {
        for &c in passport_id.as_bytes() {
            if c >= 48 && c <= 57 {} else { return false; }
        }
        true
    } else { false }
}

fn validate_required_fields(passport: &Passport) -> bool {
    validate_passport_id(&passport.passport_id)
        && validate_issued_year(&passport.issued_year)
        && validate_expiration_year(&passport.expiration_year)
        && validate_birth_year(&passport.birth_year)
        && validate_height(&passport.height)
        && validate_hair_color(&passport.hair_color)
        && validate_eye_color(&passport.eye_color)
}

fn load_input(filename: &str) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() {
    let filename = "input.txt";
    let reader = load_input(filename);

    let mut counter_1 = 0;
    let mut counter_2 = 0;

    let mut passport: Passport = Passport::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(|c| (c == ':') || (c == ' ')).collect();

        if line.is_empty() {
            if contains_required_fields(&passport) {
                counter_1 += 1;
                if validate_required_fields(&passport) {
                    counter_2 += 1;
                }
            }

            passport = Passport::new();
        } else {
            for (i, word) in split.iter().enumerate() {
                match *word {
                    "byr" => passport.birth_year = String::from(*split.get(i + 1).unwrap()),
                    "iyr" => passport.issued_year = String::from(*split.get(i + 1).unwrap()),
                    "eyr" => passport.expiration_year = String::from(*split.get(i + 1).unwrap()),
                    "hgt" => passport.height = String::from(*split.get(i + 1).unwrap()),
                    "hcl" => passport.hair_color = String::from(*split.get(i + 1).unwrap()),
                    "ecl" => passport.eye_color = String::from(*split.get(i + 1).unwrap()),
                    "pid" => passport.passport_id = String::from(*split.get(i + 1).unwrap()),
                    _ => ()
                };
            }
        }
    }
    // Need to process last passport--last line of file is not blank so above won't trigger processing
    if contains_required_fields(&passport) {
        counter_1 += 1;
        if validate_required_fields(&passport) {
            counter_2 += 1;
        }
    }

    println!("# of valid passports for first set of criteria: {}", counter_1);
    println!("# of valid passports for second set of criteria: {}", counter_2);
}
