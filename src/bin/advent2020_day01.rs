use itertools::Itertools;
use std::num::ParseIntError;

fn get_data() -> Result<Vec<u32>, ParseIntError> {
    let input = include_str!("../../inputs/advent2020_day01_input.txt");
    input.lines().map(|line| line.parse::<u32>()).collect()
}

fn find_combination(target_value: u32, data: &Vec<u32>, k: usize) -> Result<Vec<u32>, String> {
    for combination in Itertools::combinations(data.iter().copied(), k) {
        if combination.iter().sum::<u32>() == target_value {
            return Ok(combination);
        }
    }
    Err(String::from(
        "No combinations found that match the target value!",
    ))
}

fn main() {
    let target_value = 2020;
    let data = get_data().expect("Error parsing data!");
    let part_one_combination =
        find_combination(target_value, &data, 2).expect("Couldn't find a solution for part one!");
    println!(
        "Part one: {}",
        part_one_combination.iter().fold(1, |acc, x| acc * x)
    );
    let part_two_combination =
        find_combination(target_value, &data, 3).expect("Couldn't find a solution for part two!");
    println!(
        "Part two: {}",
        part_two_combination.iter().fold(1, |acc, x| acc * x)
    );
}
