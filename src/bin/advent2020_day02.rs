use scan_fmt::parse::ScanError;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Rule {
    low: usize,
    high: usize,
    char: char,
    pass: String,
}

fn get_data() -> Result<Vec<Rule>, ScanError> {
    let input = include_str!("../../inputs/advent2020_day02_input.txt");
    input
        .lines()
        .map(|line| {
            let (low, high, char, pass) =
                scan_fmt!(line, "{d}-{d} {}: {}", usize, usize, char, String)?;
            Ok(Rule {
                low,
                high,
                char,
                pass,
            })
        })
        .collect()
}

fn process_rules(rules: &Vec<Rule>, part_one: bool) -> usize {
    let mut num_valid = 0;
    for rule in rules {
        if part_one {
            let num_in_pass = rule.pass.matches(rule.char).count();
            if (rule.low <= num_in_pass) && (num_in_pass <= rule.high) {
                num_valid += 1;
            }
        } else {
            let low_char = rule.pass.chars().nth(rule.low - 1).unwrap();
            let high_char = rule.pass.chars().nth(rule.high - 1).unwrap();
            if (rule.char == low_char) ^ (rule.char == high_char) {
                num_valid += 1
            }
        }
    }
    num_valid
}

fn main() {
    let rules = get_data().expect("Couldn't parse input data!");
    let part_one_valid = process_rules(&rules, true);
    println!("Part one: {}", part_one_valid);
    let part_two_valid = process_rules(&rules, false);
    println!("Part two: {}", part_two_valid);
}
