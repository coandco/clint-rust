use std::collections::HashSet;

#[derive(Debug)]
pub struct Group {
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
        if self.answers.is_empty() {
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
            .map(|line| line.chars().collect::<HashSet<_>>())
            .collect(),
    }
}

pub fn generator(input: &str) -> Vec<Group> {
    input.split("\n\n").map(str_to_group).collect()
}

pub fn part_one(data: &[Group]) -> usize {
    data.iter()
        .map(|x| x.any_questions_answered())
        .sum::<usize>()
}

pub fn part_two(data: &[Group]) -> usize {
    data.iter()
        .map(|x| x.all_questions_answered())
        .sum::<usize>()
}
