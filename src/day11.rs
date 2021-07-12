use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    y: i32,
    x: i32,
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

static NEIGHBORS: [Coord; 8] = [
    Coord { x: -1, y: -1 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: -1, y: 1 },
    Coord { x: 0, y: 1 },
    Coord { x: 1, y: 1 },
];

fn in_bounds(shape: (i32, i32), coord: Coord) -> bool {
    0 <= coord.y && coord.y < shape.0 && 0 <= coord.x && coord.x < shape.1
}

type HashGrid = HashMap<Coord, bool>;
type Neighbors = HashMap<Coord, HashSet<Coord>>;

#[derive(Debug, Clone)]
pub struct HashedGrid {
    height: i32,
    width: i32,
    grid: HashGrid,
    neighbors: Neighbors,
    crowd_tolerance: i32,
}

impl HashedGrid {
    fn new(data: &[Vec<u8>], simple_neighbors: bool) -> Self {
        let height = data.len() as i32;
        let width = data[0].len() as i32;
        let grid = Self::populate_hashgrid(data);
        let neighbors: Neighbors = match simple_neighbors {
            true => Self::generate_part_one_neighbors_hashgrid(&grid),
            false => Self::generate_part_two_neighbors_hashgrid(&grid, &height, &width),
        };
        let crowd_tolerance: i32 = if simple_neighbors { 4 } else { 5 };

        HashedGrid {
            height,
            width,
            grid,
            neighbors,
            crowd_tolerance,
        }
    }

    fn populate_hashgrid(data: &[Vec<u8>]) -> HashGrid {
        let mut grid: HashGrid = HashMap::new();
        for (i, line) in data.iter().enumerate() {
            for (j, byte) in line.iter().enumerate() {
                match *byte {
                    b'#' => grid.insert(
                        Coord {
                            y: i as i32,
                            x: j as i32,
                        },
                        true,
                    ),
                    b'L' => grid.insert(
                        Coord {
                            y: i as i32,
                            x: j as i32,
                        },
                        false,
                    ),
                    _ => Some(true), // Ignore floor tiles and don't have them in the grid
                };
            }
        }
        grid
    }

    fn generate_part_one_neighbors_hashgrid(grid: &HashGrid) -> Neighbors {
        let mut neighbors: Neighbors = HashMap::new();
        for coord in grid.keys() {
            let mut neighbor_set: HashSet<Coord> = HashSet::new();
            for offset in NEIGHBORS.iter() {
                if grid.contains_key(&(*coord + *offset)) {
                    neighbor_set.insert(*coord + *offset);
                }
            }
            neighbors.insert(*coord, neighbor_set);
        }
        neighbors
    }

    fn generate_part_two_neighbors_hashgrid(
        grid: &HashGrid,
        height: &i32,
        width: &i32,
    ) -> Neighbors {
        let mut neighbors: Neighbors = HashMap::new();
        for coord in grid.keys() {
            let mut neighbor_set: HashSet<Coord> = HashSet::new();
            for offset in NEIGHBORS.iter() {
                let mut current_coord = *coord + *offset;
                while in_bounds((*height, *width), current_coord) {
                    if grid.contains_key(&current_coord) {
                        neighbor_set.insert(current_coord);
                        break;
                    }
                    current_coord += *offset;
                }
            }
            neighbors.insert(*coord, neighbor_set);
        }
        neighbors
    }

    fn _new_value(&self, coord: Coord) -> bool {
        let num_neighbors: i32 = self.neighbors[&coord]
            .iter()
            .map(|n| self.grid[n] as i32)
            .sum::<i32>();
        if num_neighbors == 0 {
            return true;
        } else if num_neighbors >= self.crowd_tolerance {
            return false;
        }
        self.grid[&coord]
    }

    fn convolute(&mut self) -> bool {
        let mut new_grid: HashGrid = HashMap::new();
        for coord in self.grid.keys() {
            new_grid.insert(*coord, self._new_value(*coord));
        }
        let any_changes: bool = self.grid != new_grid;
        self.grid = new_grid;
        any_changes
    }

    fn count_occupied(&self) -> usize {
        // Filter for true values
        self.grid.values().copied().filter(|x| *x).count()
    }

    fn run_until_finished(&mut self) -> usize {
        while self.convolute() {}
        self.count_occupied()
    }
}

type VecGrid = Vec<Vec<u8>>;
type VecNeighbors = HashMap<Coord, Vec<Coord>>;

#[derive(Debug, Clone)]
pub struct VectorGrid {
    height: i32,
    width: i32,
    grid: VecGrid,
    neighbors: VecNeighbors,
    crowd_tolerance: i32,
}

impl VectorGrid {
    fn new(grid: VecGrid, simple_neighbors: bool) -> Self {
        let height: i32 = grid.len() as i32;
        let width: i32 = grid[0].len() as i32;
        let neighbors: VecNeighbors = match simple_neighbors {
            true => Self::generate_part_one_neighbors_vecgrid(&grid),
            false => Self::generate_part_two_neighbors_vecgrid(&grid),
        };
        let crowd_tolerance: i32 = if simple_neighbors { 4_i32 } else { 5_i32 };

        VectorGrid {
            height,
            width,
            grid,
            neighbors,
            crowd_tolerance,
        }
    }

    fn generate_part_one_neighbors_vecgrid(grid: &[Vec<u8>]) -> VecNeighbors {
        let mut neighbors: VecNeighbors = HashMap::new();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == b'L' {
                    let mut neighbor_set: Vec<Coord> = vec![];
                    for offset in NEIGHBORS.iter() {
                        let new_coord: Coord = Coord {
                            y: i as i32,
                            x: j as i32,
                        } + *offset;
                        if in_bounds((height, width), new_coord)
                            && grid[new_coord.y as usize][new_coord.x as usize] == b'L'
                        {
                            neighbor_set.push(new_coord);
                        }
                    }
                    neighbors.insert(
                        Coord {
                            y: i as i32,
                            x: j as i32,
                        },
                        neighbor_set,
                    );
                }
            }
        }
        neighbors
    }

    fn generate_part_two_neighbors_vecgrid(grid: &[Vec<u8>]) -> VecNeighbors {
        let mut neighbors: VecNeighbors = HashMap::new();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        for (i, line) in grid.iter().enumerate() {
            for (j, char) in line.iter().enumerate() {
                if *char == b'L' {
                    let mut neighbor_set: Vec<Coord> = vec![];
                    for offset in NEIGHBORS.iter() {
                        let mut current_coord = Coord {
                            y: i as i32,
                            x: j as i32,
                        } + *offset;
                        while in_bounds((height, width), current_coord) {
                            if grid[current_coord.y as usize][current_coord.x as usize] == b'L' {
                                neighbor_set.push(current_coord);
                                break;
                            }
                            current_coord += *offset;
                        }
                    }
                    neighbors.insert(
                        Coord {
                            y: i as i32,
                            x: j as i32,
                        },
                        neighbor_set,
                    );
                }
            }
        }
        neighbors
    }

    #[inline]
    fn get(&self, coord: &Coord) -> u8 {
        self.grid[coord.y as usize][coord.x as usize]
    }

    #[inline]
    fn flip(&mut self, coord: &Coord) {
        self.grid[coord.y as usize][coord.x as usize] = match self.get(coord) {
            b'#' => b'L',
            b'L' => b'#',
            other => other,
        }
    }

    fn _needs_to_change(&self, coord: &Coord) -> bool {
        let num_neighbors: i32 = self.neighbors[&coord]
            .iter()
            .filter(|n| self.get(n) == b'#')
            .count() as i32;
        let current_val = self.get(coord);
        let needs_to_turn_on = current_val == b'L' && num_neighbors == 0;
        let needs_to_turn_off = current_val == b'#' && num_neighbors >= self.crowd_tolerance;
        needs_to_turn_off || needs_to_turn_on
    }

    fn convolute(&mut self) -> bool {
        // self.neighbors.keys() is an iterator of all spots we need to check for changes
        let changes: Vec<Coord> = self
            .neighbors
            .keys()
            .copied()
            .filter(|coord| self._needs_to_change(&coord))
            .collect();
        for change in &changes {
            self.flip(change)
        }
        !changes.is_empty()
    }

    fn count_occupied(&self) -> usize {
        self.neighbors
            .keys()
            .filter(|coord| self.get(coord) == b'#')
            .count()
    }

    fn run_until_finished(&mut self) -> usize {
        while self.convolute() {}
        self.count_occupied()
    }
}

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

pub fn part_one_hashgrid(data: &[Vec<u8>]) -> usize {
    let mut grid = HashedGrid::new(data, true);
    grid.run_until_finished()
}

pub fn part_two_hashgrid(data: &[Vec<u8>]) -> usize {
    let mut grid = HashedGrid::new(data, false);
    grid.run_until_finished()
}

pub fn part_one_vecgrid(data: &[Vec<u8>]) -> usize {
    let raw_grid: VecGrid = data.to_owned();
    let mut grid = VectorGrid::new(raw_grid, true);
    grid.run_until_finished()
}

pub fn part_two_vecgrid(data: &[Vec<u8>]) -> usize {
    let raw_grid: VecGrid = data.to_owned();
    let mut grid = VectorGrid::new(raw_grid, false);
    grid.run_until_finished()
}
