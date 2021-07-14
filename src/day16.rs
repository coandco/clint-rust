use hashbrown::HashMap;
use im::{HashSet, Vector};
use itertools::Itertools;
use scan_fmt::parse::ScanError;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Constraint {
    name: String,
    valid_values: HashSet<usize>,
}

impl Constraint {
    fn validate(&self, value: &usize) -> bool {
        self.valid_values.contains(value)
    }
}

impl Hash for Constraint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Constraint {}

impl FromStr for Constraint {
    type Err = ScanError;
    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let (name, range1_low, range1_high, range2_low, range2_high) = scan_fmt!(
            record,
            "{[^:]}: {d}-{d} or {d}-{d}",
            String,
            usize,
            usize,
            usize,
            usize
        )?;
        let valid_values: HashSet<usize> = HashSet::from_iter(range1_low..=range1_high)
            + HashSet::from_iter(range2_low..=range2_high);
        Ok(Self { name, valid_values })
    }
}

#[derive(Debug, Clone)]
pub struct Ticket(Vector<usize>);

impl Ticket {
    fn validate(&self, valid_values: &HashSet<usize>) -> bool {
        self.0.iter().all(|x| valid_values.contains(x))
    }

    fn sum_invalid_values(&self, valid_values: &HashSet<usize>) -> usize {
        self.0.iter().filter(|x| !valid_values.contains(x)).sum()
    }
}

impl FromStr for Ticket {
    type Err = ParseIntError;
    fn from_str(record: &str) -> Result<Self, Self::Err> {
        let values: Vector<usize> = record
            .split(',')
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vector<usize>, _>>()?;
        Ok(Self(values))
    }
}

type InputData = (Vector<Constraint>, Ticket, Vector<Ticket>);

pub fn generator(input: &str) -> InputData {
    let mut sections = input.split("\n\n");
    // Step one: ingest constraints
    let raw_constraints = sections.next().unwrap();
    let constraints: Vector<Constraint> = raw_constraints
        .lines()
        .map(|line| {
            line.parse::<Constraint>()
                .unwrap_or_else(|_| panic!("Couldn't parse constraints for line {}!", line))
        })
        .collect();

    let raw_your_ticket = sections.next().unwrap().lines().nth(1).unwrap();
    let your_ticket: Ticket = raw_your_ticket
        .parse::<Ticket>()
        .unwrap_or_else(|_| panic!("Couldn't parse your ticket ({})", raw_your_ticket));

    let raw_nearby_tickets = sections.next().unwrap();
    let nearby_tickets: Vector<Ticket> = raw_nearby_tickets
        .lines()
        .skip(1)
        .map(|line| {
            line.parse::<Ticket>()
                .unwrap_or_else(|_| panic!("Couldn't parse ticket {}", line))
        })
        .collect();
    (constraints, your_ticket, nearby_tickets)
}

pub fn part_one(data: &InputData) -> usize {
    let (constraints, _, nearby_tickets) = data;
    let all_valid_values = HashSet::unions(constraints.iter().map(|x| x.valid_values.clone()));
    nearby_tickets
        .iter()
        .map(|x| x.sum_invalid_values(&all_valid_values))
        .sum()
}

fn get_possible_fields(
    constraints: &Vector<Constraint>,
    values: &Vector<usize>,
) -> HashSet<Constraint> {
    HashSet::from_iter(
        constraints
            .iter()
            .cloned()
            .filter(|c| values.iter().all(|v| c.validate(v))),
    )
}

fn map_fields(
    constraints: &Vector<Constraint>,
    nearby_tickets: &Vector<Ticket>,
) -> HashMap<usize, String> {
    // For each index, find the possible constraints that could apply to it, then sort the list by the number of constraints
    let mut sorted_fields: Vector<(usize, HashSet<Constraint>)> = (0..nearby_tickets[0].0.len())
        .map(|i| {
            let values_in_field: Vector<usize> = nearby_tickets.iter().map(|x| x.0[i]).collect();
            (i, get_possible_fields(&constraints, &values_in_field))
        })
        .sorted_unstable_by(|a, b| Ord::cmp(&a.1.len(), &b.1.len()))
        .collect();

    let mut solved_fields: HashMap<usize, String> = HashMap::new();
    while !sorted_fields.is_empty() {
        // Take the first (and smallest) item from sorted_fields
        let (i, possible_constraints) = sorted_fields.pop_front().unwrap();
        // We should always have a single known constraint for the leftmost field
        assert_eq!(possible_constraints.len(), 1);
        // Get that single constraint
        let solved_constraint = possible_constraints.iter().next().unwrap();
        // Associate it with the correct index in our solved fields hashmap
        solved_fields.insert(i, solved_constraint.name.clone());
        // Remove the constraint as a possibility from all of the other indices now that we've solved it
        for i in 0..sorted_fields.len() {
            sorted_fields[i].1.remove(solved_constraint);
        }
    }
    solved_fields
}

pub fn part_two(data: &InputData) -> usize {
    let (constraints, your_ticket, nearby_tickets) = data;
    let all_valid_values = HashSet::unions(constraints.iter().map(|x| x.valid_values.clone()));
    let valid_nearby_tickets: Vector<Ticket> = nearby_tickets
        .iter()
        .cloned()
        .filter(|x| x.validate(&all_valid_values))
        .collect();
    let field_indices = map_fields(&constraints, &valid_nearby_tickets);
    let your_departure_values: Vector<usize> = your_ticket
        .0
        .iter()
        .enumerate()
        .filter_map(|(i, value)| {
            if field_indices.get(&i).unwrap().starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .copied()
        .collect();
    your_departure_values.iter().product()
}
