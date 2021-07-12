use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    y: usize,
    x: usize,
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

fn has_tree(treemap: &[Vec<u8>], loc: &Coord) -> Option<bool> {
    if loc.y >= treemap.len() {
        return None;
    }
    if treemap[loc.y][loc.x % treemap[0].len()] == b'#' {
        Some(true)
    } else {
        Some(false)
    }
}

fn check_slope(treemap: &[Vec<u8>], slope: Coord) -> usize {
    let mut num_trees = 0;
    let mut curpos = Coord { x: 0, y: 0 };
    while let Some(current_tree) = has_tree(treemap, &curpos) {
        if current_tree {
            num_trees += 1
        }
        curpos += slope
    }
    num_trees
}

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn part_one(data: &[Vec<u8>]) -> usize {
    check_slope(&data, Coord { x: 3, y: 1 })
}

pub fn part_two(data: &[Vec<u8>]) -> usize {
    let slopes = vec![
        Coord { x: 1, y: 1 },
        Coord { x: 3, y: 1 },
        Coord { x: 5, y: 1 },
        Coord { x: 7, y: 1 },
        Coord { x: 1, y: 2 },
    ];
    let trees_per_slope: Vec<usize> = slopes
        .iter()
        .map(|slope| check_slope(&data, *slope))
        .collect();
    trees_per_slope.iter().product()
}
