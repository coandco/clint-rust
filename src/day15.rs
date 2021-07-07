use hashbrown::HashMap;

fn run_game(data: &Vec<usize>, iterations: usize) -> usize {
    let mut seen: HashMap<usize, usize> = data
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i+1 as usize))
        .collect();
    let mut last_number: usize = *data.last().unwrap();
    for i in data.len()..iterations {
        // HashMap.insert(foo) returns Some(old-value) if it exists or None if it doesn't
        // By saying unwrap_or(i) there, we're setting it to zero if it's a number we haven't seen before
        last_number = i - seen.insert(last_number, i).unwrap_or(i);
    }
    last_number
}

pub fn generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn part_one(data: &Vec<usize>) -> usize {
    run_game(data, 2020)
}

pub fn part_two(data: &Vec<usize>) -> usize {
    run_game(data, 30000000)
}
