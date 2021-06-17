use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
struct Coord {
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

fn get_data() -> Vec<Vec<u8>> {
    let input = include_str!("../../inputs/advent2020_day03_input.txt");
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn has_tree(treemap: &Vec<Vec<u8>>, loc: &Coord) -> Option<bool> {
    if loc.y >= treemap.len() {
        return None;
    }
    return if treemap[loc.y][loc.x % treemap[0].len()] == b'#' {
        Some(true)
    } else {
        Some(false)
    };
}

fn check_slope(treemap: &Vec<Vec<u8>>, slope: Coord) -> usize {
    let mut num_trees = 0;
    let mut curpos = Coord { x: 0, y: 0 };
    loop {
        if let Some(current_tree) = has_tree(treemap, &curpos) {
            if current_tree {
                num_trees += 1
            }
            curpos += slope
        } else {
            break;
        }
    }
    num_trees
}

fn main() {
    let treemap = get_data();
    let part_one_solution = check_slope(&treemap, Coord { x: 3, y: 1 });
    println!("Part one: {}", part_one_solution);
    let slopes = vec![
        Coord { x: 1, y: 1 },
        Coord { x: 3, y: 1 },
        Coord { x: 5, y: 1 },
        Coord { x: 7, y: 1 },
        Coord { x: 1, y: 2 },
    ];
    let trees_per_slope: Vec<usize> = slopes
        .iter()
        .map(|slope| check_slope(&treemap, *slope))
        .collect();
    let part_two_solution = trees_per_slope.iter().fold(1, |acc, x| acc * x);
    println!("Part two: {}", part_two_solution);
}
