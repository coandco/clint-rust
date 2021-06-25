use itertools::Itertools;

fn find_invalid(data: &Vec<usize>, preamble_length: usize) -> Option<usize> {
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

fn find_summed_range(data: &Vec<usize>, target_number: usize) -> Option<Vec<usize>> {
    for i in 0..data.len() {
        let mut total = data[i];
        for j in i + 1..=data.len() {
            total += data[j];
            if total == target_number {
                return Some(data[i..j].to_vec());
            } else if total > target_number {
                break;
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

pub fn part_one(data: &Vec<usize>) -> usize {
    find_invalid(data, 25).unwrap()
}

pub fn part_two(data: &Vec<usize>) -> usize {
    let invalid_num = find_invalid(data, 25).unwrap();
    let good_range = find_summed_range(data, invalid_num).unwrap();
    let (min, max) = Itertools::minmax(good_range.iter()).into_option().unwrap();
    min + max
}
