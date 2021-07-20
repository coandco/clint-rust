use hashbrown::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Rule {
    id: usize,
    chains: Vec<Vec<usize>>,
    matches: HashSet<String>,
    fully_known: bool,
}

impl Rule {
    fn can_be_known(&self, known_rule_ids: &HashSet<usize>) -> bool {
        for chain in self.chains.iter() {
            for item in chain {
                if !known_rule_ids.contains(item) {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Rule {
    type Err = String;
    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let (id_str, rule_text) = record.split_once(": ").ok_or("Couldn't find rule text")?;
        let id: usize = id_str
            .parse()
            .map_err(|_| format!("Couldn't parse id {}", id_str))?;
        if rule_text.starts_with('"') {
            let matches_str = rule_text.replace('"', "");
            let mut matches: HashSet<String> = HashSet::new();
            matches.insert(matches_str);
            Ok(Rule {
                id,
                chains: vec![],
                matches,
                fully_known: true,
            })
        } else {
            let chains = rule_text
                .split(" | ")
                .map(|chunk| {
                    chunk
                        .split(' ')
                        .map(|rnum| {
                            rnum.parse::<usize>()
                                .unwrap_or_else(|_| panic!("Couldn't parse {} as a number", chunk))
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            let matches: HashSet<String> = HashSet::new();
            Ok(Rule {
                id,
                chains,
                matches,
                fully_known: false,
            })
        }
    }
}

fn fill_out_rules(rules: &mut HashMap<usize, Rule>) {
    loop {
        let known_rule_ids: HashSet<usize> = rules
            .iter()
            .filter_map(|(k, v)| {
                if v.fully_known {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect();
        if known_rule_ids.len() >= rules.len() {
            break;
        }

        let rules_what_can_be_known: Vec<usize> = rules
            .iter()
            .filter_map(|(k, v)| {
                if !v.fully_known && v.can_be_known(&known_rule_ids) {
                    Some(*k)
                } else {
                    None
                }
            }).collect();

        // if there's a recursive rule, this will stop the loop before trying to recurse
        if rules_what_can_be_known.is_empty() {
            break;
        }

        for candidate in rules_what_can_be_known {
            // For each rule, attempt to build a set of all possible strings it matches
            let mut full_matches: HashSet<String> = HashSet::new();
            for chain in rules[&candidate].chains.iter() {
                // Process each or'd chain separately, then combine them at the end
                let mut chain_texts: HashSet<String> = HashSet::new();
                for ruleid in chain {
                    if chain_texts.is_empty() {
                        // Start by copying the first chained rule's string set whole
                        chain_texts = rules.get(ruleid).unwrap().matches.clone();
                    } else {
                        // Once we have something to start with, start appending other elements to the strings.
                        // Using chain_texts.clone() to make a copy so it won't complain about us modifying the thing
                        // we're iterating through.
                        for text in chain_texts.clone() {
                            // Get rid of the old incomplete version before we start appending
                            chain_texts.remove(text.as_str());
                            for rule_match in rules[ruleid].matches.iter() {
                                chain_texts.insert(format!("{}{}", text, rule_match));
                            }
                        }
                    }
                }
                full_matches.extend(chain_texts);
            }
            let candidate_rule = rules.entry(candidate).or_default();
            candidate_rule.matches = full_matches;
            candidate_rule.fully_known = true;
        }
    }
}

fn check_text_part_two(text: &str, rules: &HashMap<usize, Rule>) -> bool {
    // First off, everything is done in chunks of 8, so early-invalidate anything that isn't a multiple of 8 long
    let chunk_length = rules[&42].matches.iter().next().unwrap().len();
    if text.len() % chunk_length != 0 {
        return false;
    }
    // The rule is that there has to be at least one 42-set at the beginning, followed by a matched number of 42 and 31
    // The algorithm, therefore, is to start at the end, match 31 sets backwards, and then once we've stopped match
    // 42 sets until we've hit at least n+1 of them
    let reversed_chunks: Vec<&str> = text
        .as_bytes()
        .chunks(chunk_length)
        .rev()
        .map(|chunk| {
            std::str::from_utf8(chunk).unwrap()
        })
        .collect();
    let mut num_31s: usize = 0;
    for chunk in reversed_chunks.iter() {
        if rules[&31].matches.contains(*chunk) {
            num_31s += 1;
        } else {
            break;
        }
    }
    let mut num_42s: usize = 0;
    for chunk in reversed_chunks.into_iter().skip(num_31s) {
        if rules[&42].matches.contains(chunk) {
            num_42s += 1;
        } else {
            // Once we've run out of 31s, it has to be 42s all the way to the start
            return false;
        }
    }
    // There has to be at least 1 num_31s, and num_42s has to be greater than num_31s
    1 <= num_31s && num_31s < num_42s
}

pub fn generator(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let (rules_str, texts_str) = input
        .split_once("\n\n")
        .expect("Couldn't split rules from texts!");
    // This produces empty (non-filled-out) rules that will need to be populated
    let mut rules: HashMap<usize, Rule> = rules_str
        .lines()
        .map(|line| {
            let rule = Rule::from_str(line)
                .unwrap_or_else(|e| panic!("Couldn't parse rule '{}' with error '{}'", line, e));
            (rule.id, rule)
        })
        .collect();
    fill_out_rules(&mut rules);
    let text: Vec<String> = texts_str.lines().map(str::to_string).collect();
    (rules, text)
}

pub fn part_one(data: &(HashMap<usize, Rule>, Vec<String>)) -> usize {
    let rules = data.0.clone();
    let texts = data.1.clone();
    texts
        .iter()
        .cloned()
        .filter(|text| rules[&0_usize].matches.contains(text))
        .count()
}

pub fn part_two(data: &(HashMap<usize, Rule>, Vec<String>)) -> usize {
    let rules = data.0.clone();
    let texts = data.1.clone();
    texts
        .iter()
        .cloned()
        .filter(|text| check_text_part_two(text, &rules))
        .count()
}
