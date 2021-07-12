use itertools::Itertools;
use std::cmp::Ordering;

fn find_invalid(data: &[usize], preamble_length: usize) -> Option<usize> {
    let mut known = data[..preamble_length].to_vec();
    for item in &data[preamble_length..] {
        let mut item_valid = false;
        for (a, b) in Itertools::tuple_combinations(known.iter()) {
            if (a + b) == *item {
                known.remove(0);
                known.push(*item);
                item_valid = true;
                break;
            }
        }
        if item_valid {
            continue;
        } else {
            return Some(*item);
        }
    }
    None
}

fn find_summed_range(data: &[usize], target_number: usize) -> Option<Vec<usize>> {
    for i in 0..data.len() {
        let mut total = data[i];
        for j in i + 1..=data.len() {
            total += data[j];
            match total.cmp(&target_number) {
                Ordering::Equal => return Some(data[i..j].to_vec()),
                Ordering::Greater => break,
                _ => ()
            }
        }
    }
    None
}

pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .expect("Failed to parse input!")
}

pub fn part_one(data: &[usize]) -> usize {
    find_invalid(data, 25).unwrap()
}

pub fn part_two(data: &[usize]) -> usize {
    let invalid_num = find_invalid(data, 25).unwrap();
    let good_range = find_summed_range(data, invalid_num).unwrap();
    let (min, max) = Itertools::minmax(good_range.iter()).into_option().unwrap();
    min + max
}
