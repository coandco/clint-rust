use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, Debug)]
#[display("{low}-{high} {char}: {pass}")]
pub struct Rule {
    low: usize,
    high: usize,
    char: char,
    pass: String,
}

pub fn generator(input: &str) -> Vec<Rule> {
    input.lines().map(|line| line.parse::<Rule>()).collect::<Result<Vec<Rule>, _>>().expect("Error parsing input for day 2!")
}

pub fn part_one(data: &Vec<Rule>) -> usize {
    let mut num_valid = 0;
    for rule in data {
        let num_in_pass = rule.pass.matches(rule.char).count();
        if (rule.low <= num_in_pass) && (num_in_pass <= rule.high) {
            num_valid += 1;
        }
    }
    num_valid
}

pub fn part_two(data: &Vec<Rule>) -> usize {
    let mut num_valid = 0;
    for rule in data {
        let low_char = rule.pass.chars().nth(rule.low - 1).unwrap();
        let high_char = rule.pass.chars().nth(rule.high - 1).unwrap();
        if (rule.char == low_char) ^ (rule.char == high_char) {
            num_valid += 1
        }
    }
    num_valid
}