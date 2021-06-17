#[macro_use(scan_fmt)]
extern crate scan_fmt;
use hex::decode as hex_decode;
use scan_fmt::parse::ScanError;
use std::collections::HashMap;
use std::str::FromStr;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn matches_year_range(year_str: &str, low: usize, high: usize) -> bool {
    if year_str.len() != 4 {
        return false;
    }
    return if let Ok(year_int) = year_str.parse::<usize>() {
        low <= year_int && year_int <= high
    } else {
        false
    };
}

fn matches_height(raw_height: &str) -> bool {
    let height_len = raw_height.len();
    let (height_str, height_suffix) = raw_height.split_at(height_len - 2);
    return if let Ok(height_int) = height_str.parse::<usize>() {
        match height_suffix {
            "cm" => 150 <= height_int && height_int <= 193,
            "in" => 59 <= height_int && height_int <= 76,
            _ => false,
        }
    } else {
        false
    };
}

#[derive(Debug)]
struct Passport(HashMap<String, String>);
impl Passport {
    fn part_one_valid(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|key| self.0.contains_key(*key))
    }

    fn part_two_valid(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|key| {
            if let Some(value) = self.0.get(*key) {
                match *key {
                    "byr" => matches_year_range(&value, 1920, 2002),
                    "iyr" => matches_year_range(&value, 2010, 2020),
                    "eyr" => matches_year_range(&value, 2020, 2030),
                    "hgt" => matches_height(&value),
                    "hcl" => value.len() == 7 && hex_decode(&value[1..]).is_ok(),
                    "ecl" => matches!(
                        value.as_str(),
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    ),
                    "pid" => value.len() == 9 && value.parse::<usize>().is_ok(),
                    _ => true,
                }
            } else {
                return false;
            }
        })
    }
}
impl FromStr for Passport {
    type Err = ScanError;
    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let mut new_passport = HashMap::new();
        for entry in record.split_whitespace() {
            let (key, value) = scan_fmt!(entry, "{}:{}", String, String)?;
            new_passport.insert(key, value);
        }
        Ok(Passport(new_passport))
    }
}

fn get_data() -> Result<Vec<Passport>, ScanError> {
    let input = include_str!("../../inputs/advent2020_day04_input.txt");
    input.split("\n\n").map(|record| record.parse()).collect()
}

fn main() {
    let data = get_data().expect("Couldn't parse input!");
    let num_valid_part_one = data.iter().filter(|record| record.part_one_valid()).count();
    println!("Part one: {}", num_valid_part_one);
    let num_valid_part_two = data.iter().filter(|record| record.part_two_valid()).count();
    println!("Part two: {}", num_valid_part_two);
}
