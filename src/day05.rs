use phf::phf_map;
use std::num::ParseIntError;

type Seat = u32;

// phf is necessary to have statically defined hashmaps
static TRANSLATE: phf::Map<char, char> = phf_map! {
    'F' => '0',
    'B' => '1',
    'L' => '0',
    'R' => '1'
};

fn translate_seat(seat: &str) -> Result<Seat, ParseIntError> {
    let mut outstring = String::with_capacity(seat.len());
    for c in seat.chars() {
        match TRANSLATE.get(&c) {
            Some(replacement) => outstring.push(*replacement),
            None => outstring.push(c),
        }
    }
    Seat::from_str_radix(outstring.as_str(), 2)
}

pub fn generator(input: &str) -> (Seat, Seat, Seat) {
    let data = input
        .lines()
        .map(translate_seat)
        .collect::<Result<Vec<Seat>, _>>()
        .expect("Error parsing input for day 5!");
    data.iter()
        .copied()
        .fold((Seat::MAX, Seat::MIN, 0), |(min, max, sum), x| {
            (min.min(x), max.max(x), sum + x)
        })
}

pub fn part_one(data: &(Seat, Seat, Seat)) -> u32 {
    let (_, max, _) = *data;
    max
}

pub fn part_two(data: &(Seat, Seat, Seat)) -> u32 {
    let (min, max, sum) = *data;
    (min..=max).sum::<Seat>() - sum
}
