use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct Group {
    answers: Vec<HashSet<char>>,
}

impl Group {
    fn any_questions_answered(&self) -> usize {
        let mut result: HashSet<char> = HashSet::new();
        for set in &self.answers {
            result = &result | set;
        }
        result.len()
    }

    fn all_questions_answered(&self) -> usize {
        if self.answers.len() == 0 {
            return 0;
        }
        let mut result: HashSet<char> = self.answers[0].clone();
        for set in &self.answers {
            result = &result & set;
        }
        result.len()
    }
}

fn str_to_group(s: &str) -> Group {
    Group {
        answers: s
            .lines()
            .map(|line| HashSet::from_iter(line.chars()))
            .collect(),
    }
}

fn get_data() -> Vec<Group> {
    let data = include_str!("../../inputs/advent2020_day06_input.txt");
    data.split("\n\n").map(str_to_group).collect()
}

fn main() {
    let sets = get_data();
    let part_one_answer = sets
        .iter()
        .map(|x| x.any_questions_answered())
        .sum::<usize>();
    println!("Part one: {}", part_one_answer);
    let part_two_answer = sets
        .iter()
        .map(|x| x.all_questions_answered())
        .sum::<usize>();
    println!("Part two: {}", part_two_answer);
}
