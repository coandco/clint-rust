use itertools::Itertools;
use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<usize> {
    // Start with just the wall socket
    let mut full_data: Vec<usize> = vec![0];
    // Parse our input and sort it
    let sorted_input: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>())
        .map(|x| x.expect("Failed to parse input!"))
        .sorted_unstable()
        .collect();
    // Append it to the full data
    full_data.extend(sorted_input);
    // Finally, add your device to the list, at max_value + 3
    full_data.push(*full_data.last().unwrap() + 3);
    full_data
}

pub fn part_one(data: &[usize]) -> usize {
    // Calculate the differences between each item
    let differences: Vec<usize> = Itertools::tuple_windows(data.iter())
        .map(|(a, b)| b - a)
        .collect();
    // Count up all the ones and threes
    let (ones, threes) = differences
        .iter()
        .fold((0, 0), |(ones, threes), x| match x {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    ones * threes as usize
}

fn calculate_value(known_cache: &mut HashMap<usize, usize>, entry: usize) -> usize {
    let mut total: usize = 0;
    for step in 1..=3_usize {
        // If value + step is in the cache, add it to your own value
        total += known_cache.get(&(entry + (step as usize))).unwrap_or(&(0_usize));
    }
    // Add this calculated value to the cache before returning
    known_cache.insert(entry, total);
    total
}

pub fn part_two(data: &[usize]) -> usize {
    // Start with our device and initialize the number of paths to it to one
    let mut known_cache: HashMap<usize, usize> = HashMap::new();
    known_cache.insert(*data.last().unwrap(), 1);
    // Go backwards through the list starting with the second-to-last item,
    // setting each item to the sum of the ones in range of it
    let mut in_range: Vec<usize> = vec![];
    for entry in data.iter().rev().skip(1) {
        in_range.push(calculate_value(&mut known_cache, *entry));
    }
    // Finally, return the calculated value for your wall socket,
    // which will now be at the end of the list because of the reversal
    *in_range.last().unwrap()
}