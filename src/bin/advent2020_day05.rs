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

fn get_data() -> Result<Vec<Seat>, ParseIntError> {
    let input = include_str!("../../inputs/advent2020_day05_input.txt");
    input.lines().map(translate_seat).collect()
}

fn main() {
    let seats = get_data().expect("Error parsing input!");
    let (min, max, sum) = seats
        .iter()
        .copied()
        .fold((Seat::MAX, Seat::MIN, 0), |(min, max, sum), x| {
            (min.min(x), max.max(x), sum + x)
        });
    println!("Part one: {}", max);
    println!("Part two: {}", (min..=max).sum::<Seat>() - sum);
}
