use itertools::Itertools;
use std::num::ParseIntError;

fn find_combination(target_value: u32, data: &[u32], k: usize) -> Result<Vec<u32>, String> {
    for combination in Itertools::combinations(data.iter().copied(), k) {
        if combination.iter().sum::<u32>() == target_value {
            return Ok(combination);
        }
    }
    Err(String::from(
        "No combinations found that match the target value!",
    ))
}


pub fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse::<u32>()).collect::<Result<Vec<u32>, ParseIntError>>().expect("Error parsing input for day 1")
}

pub fn part_one(data: &[u32]) -> u32 {
    let part_one_combination =
        find_combination(2020, &data, 2).expect("Couldn't find a solution for part one!");
    part_one_combination.iter().product()
}

pub fn part_two(data: &[u32]) -> u32 {
    let part_two_combination =
        find_combination(2020, &data, 3).expect("Couldn't find a solution for part two!");
    part_two_combination.iter().product()
}