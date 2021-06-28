use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
pub enum Op {
    N,
    E,
    S,
    W,
    R,
    L,
    F,
}

impl Op {
    pub fn from(c: char) -> Option<Self> {
        match c {
            'N' => Some(Self::N),
            'E' => Some(Self::E),
            'S' => Some(Self::S),
            'W' => Some(Self::W),
            'R' => Some(Self::R),
            'L' => Some(Self::L),
            'F' => Some(Self::F),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    pub fn from(op: &Op) -> Self {
        match op {
            Op::N => Heading::N,
            Op::E => Heading::E,
            Op::S => Heading::S,
            Op::W => Heading::W,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn turn_right(&self, times: i32) -> Heading {
        let mut new_heading: Heading = *self;
        for _ in 0..(times as usize) {
            new_heading = match new_heading {
                Heading::N => Heading::E,
                Heading::E => Heading::S,
                Heading::S => Heading::W,
                Heading::W => Heading::N,
            }
        }
        new_heading
    }

    #[inline]
    fn turn_left(&self, times: i32) -> Heading {
        let mut new_heading: Heading = *self;
        for _ in 0..(times as usize) {
            new_heading = match new_heading {
                Heading::N => Heading::W,
                Heading::E => Heading::N,
                Heading::S => Heading::E,
                Heading::W => Heading::S,
            }
        }
        new_heading
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Coord {
    y: i32,
    x: i32,
}

impl Coord {
    #[inline]
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn move_direction(&self, direction: &Heading, amount: i32) -> Self {
        let direction_unit = match direction {
            Heading::N => Coord { y: -1, x: 0 },
            Heading::E => Coord { y: 0, x: 1 },
            Heading::S => Coord { y: 1, x: 0 },
            Heading::W => Coord { y: 0, x: -1 },
        };
        *self
            + Coord {
                y: direction_unit.y * amount,
                x: direction_unit.x * amount,
            }
    }

    #[inline]
    fn rotate_right(&self, times: i32) -> Coord {
        let mut new_point: Coord = *self;
        for _ in 0..(times as usize) {
            new_point = Coord {
                x: -new_point.y,
                y: new_point.x,
            }
        }
        new_point
    }

    #[inline]
    fn rotate_left(&self, times: i32) -> Coord {
        let mut new_point: Coord = *self;
        for _ in 0..(times as usize) {
            new_point = Coord {
                x: new_point.y,
                y: -new_point.x,
            }
        }
        new_point
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

pub fn generator(input: &str) -> Vec<(Op, i32)> {
    input
        .lines()
        .map(|line| {
            let op = Op::from(line.chars().next().unwrap()).expect("Invalid op!");
            let value = &line[1..].parse::<i32>().expect("Couldn't parse value!");
            (op, *value)
        })
        .collect()
}

pub fn part_one(data: &Vec<(Op, i32)>) -> i32 {
    let mut current_loc = Coord { x: 0, y: 0 };
    let mut current_heading = Heading::E;
    for (op, value) in data {
        match op {
            Op::N | Op::S | Op::E | Op::W => {
                current_loc = current_loc.move_direction(&Heading::from(op), *value);
            }
            Op::R => {
                let num_turns = (value / 90) % 4;
                current_heading = current_heading.turn_right(num_turns);
            }
            Op::L => {
                let num_turns = (value / 90) % 4;
                current_heading = current_heading.turn_left(num_turns);
            }
            Op::F => current_loc = current_loc.move_direction(&current_heading, *value),
        }
    }
    current_loc.manhattan_distance()
}

pub fn part_two(data: &Vec<(Op, i32)>) -> i32 {
    let mut ship_loc = Coord { x: 0, y: 0 };
    let mut waypoint_loc = Coord { y: -1, x: 10 };
    for (op, value) in data {
        match op {
            Op::N | Op::S | Op::E | Op::W => {
                waypoint_loc = waypoint_loc.move_direction(&Heading::from(op), *value);
            }
            Op::R => {
                let num_rotations = (value / 90) % 4;
                waypoint_loc = waypoint_loc.rotate_right(num_rotations);
            }
            Op::L => {
                let num_rotations = (value / 90) % 4;
                waypoint_loc = waypoint_loc.rotate_left(num_rotations);
            }
            Op::F => {
                ship_loc += Coord {
                    y: waypoint_loc.y * value,
                    x: waypoint_loc.x * value,
                }
            }
        }
    }
    ship_loc.manhattan_distance()
}
