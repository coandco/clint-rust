use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Bag {
    color: String,
    bag_value: u32,
    contains: HashMap<String, u32>,
    unknown_contains: HashMap<String, u32>,
}

type BagMap = HashMap<String, Bag>;
type ReverseBagMap = HashMap<String, Vec<String>>;

fn line_to_bag(line: &str) -> Result<Bag, String> {
    let mut contains: HashMap<String, u32> = HashMap::new();
    // Using lazy_static here so that we don't have to recompile the regex every function run
    lazy_static! {
        static ref LINE_REGEX: Regex =
            Regex::new(r"(?P<color>[a-z]+ [a-z]+) bags contain (?P<contain>[^.]*)\.").unwrap();
        static ref CONTAINS_REGEX: Regex =
            Regex::new(r"(?P<num>\d+) (?P<color>[a-z]+ [a-z]+) bags?").unwrap();
    }
    if LINE_REGEX.is_match(line) {
        let captures = LINE_REGEX.captures(line).unwrap();
        let color = captures.name("color").unwrap().as_str().to_string();
        for item in captures.name("contain").unwrap().as_str().split(",") {
            if item == "no other bags" {
                continue;
            } else {
                if CONTAINS_REGEX.is_match(item) {
                    let captures = CONTAINS_REGEX.captures(item).unwrap();
                    let contains_num = captures.name("num").unwrap().as_str().to_string();
                    let contains_color = captures.name("color").unwrap().as_str().to_string();
                    contains.insert(contains_color, contains_num.parse::<u32>().unwrap());
                } else {
                    return Err(format!("Couldn't parse contains string {}", item));
                }
            }
        }
        Ok(Bag {
            color: color,
            bag_value: 0,
            contains: contains.clone(),
            unknown_contains: contains.clone(),
        })
    } else {
        Err(format!("Couldn't parse line {}", line))
    }
}

fn build_reversed_bag_map(bag_map: &BagMap) -> ReverseBagMap {
    let mut reversed_bag_map: ReverseBagMap = HashMap::new();
    for color in bag_map.keys() {
        let containing_colors = bag_map
            .values()
            .filter_map(|bag| {
                if bag.contains.keys().contains(color) {
                    Some(bag.color.clone())
                } else {
                    None
                }
            })
            .collect();
        reversed_bag_map.insert(color.clone(), containing_colors);
    }
    reversed_bag_map
}

fn get_all_containing_bags(reversed_bag_map: &ReverseBagMap, to_find: String) -> HashSet<String> {
    let mut current_set: HashSet<String> = HashSet::new();
    current_set.insert(to_find.clone());
    loop {
        let start_set = current_set.clone();
        for color in &start_set {
            let new_bags = reversed_bag_map.get(color.as_str()).unwrap();
            current_set.extend(new_bags.iter().cloned())
        }
        if start_set == current_set {
            return current_set;
        }
    }
}

fn bag_descendants(bag_map: &BagMap, start_bag: String) -> HashSet<String> {
    let mut descendants: HashSet<String> = HashSet::new();
    descendants.insert(start_bag.clone());
    loop {
        let start_set = descendants.clone();
        for color in &start_set {
            let new_bags = &bag_map.get(color.as_str()).unwrap().contains;
            descendants.extend(new_bags.keys().cloned())
        }
        if start_set == descendants {
            return descendants;
        }
    }
}

fn generate_bag_values(
    bag_map: &mut BagMap,
    reversed_bag_map: &ReverseBagMap,
) -> HashMap<String, u32> {
    let mut known_values: HashMap<String, u32> = HashMap::new();
    for bag in bag_map.values() {
        if bag.unknown_contains.len() == 0 {
            known_values.insert(bag.color.clone(), bag.bag_value);
        }
    }
    loop {
        let this_pass: Vec<String> = known_values.keys().cloned().collect();
        for color in &this_pass {
            for parent_color in &reversed_bag_map[color] {
                if known_values.contains_key(parent_color.as_str()) {
                    continue;
                }
                if !bag_map.contains_key(parent_color.as_str()) {
                    continue;
                }
                let mut parent_bag = bag_map.get_mut(parent_color.as_str()).unwrap();
                if parent_bag.unknown_contains.contains_key(color) {
                    // Add the known value times the number it contains
                    let new_value = parent_bag.contains[color]
                        + (known_values[color] * parent_bag.contains[color]);
                    parent_bag.bag_value += new_value;
                    parent_bag.unknown_contains.remove(color);
                    if parent_bag.unknown_contains.len() == 0 {
                        known_values.insert(parent_bag.color.clone(), parent_bag.bag_value);
                    }
                }
            }
        }
        if known_values.len() == this_pass.len() {
            break;
        }
    }
    known_values.clone()
}

pub fn generator(input: &str) -> (BagMap, ReverseBagMap) {
    let bag_list = input
        .lines()
        .map(line_to_bag)
        .collect::<Result<Vec<Bag>, String>>()
        .expect("Couldn't parse input for day 7!");
    let mut bag_map: BagMap = HashMap::new();
    for bag in bag_list {
        bag_map.insert(bag.color.clone(), bag.to_owned());
    }
    let reversed_bag_map = build_reversed_bag_map(&bag_map);
    (bag_map, reversed_bag_map)
}

pub fn part_one(data: &(BagMap, ReverseBagMap)) -> usize {
    let (_, reversed_bag_map) = data;
    get_all_containing_bags(reversed_bag_map, "shiny gold".to_string()).len() - 1
}

pub fn part_two(data: &(BagMap, ReverseBagMap)) -> u32 {
    let (bag_map, reversed_bag_map) = data;
    let mut my_bag_map = bag_map.clone();
    let relevant_bags = bag_descendants(&bag_map, "shiny gold".to_string());
    my_bag_map.retain(|_, bag| relevant_bags.contains(&bag.color));
    let known_bag_values = generate_bag_values(&mut my_bag_map, reversed_bag_map);
    known_bag_values[&"shiny gold".to_string()]
}
