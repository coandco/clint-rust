use derive_more::{Add, AddAssign};
use hashbrown::{HashMap, HashSet};
use std::fmt::Debug;

pub trait NewXY {
    fn new_xy(x: i32, y: i32) -> Self;
}

pub trait Neighbors {
    fn neighbors(&self) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Add, AddAssign)]
pub struct Coord2D {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Add, AddAssign)]
pub struct Coord3D {
    x: i32,
    y: i32,
    z: i32,
}

impl NewXY for Coord3D {
    fn new_xy(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }
}

impl Neighbors for Coord3D {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors_3d: Vec<Coord3D> = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    neighbors_3d.push(*self + Coord3D { x, y, z })
                }
            }
        }
        neighbors_3d
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Add, AddAssign)]
pub struct Coord4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl NewXY for Coord4D {
    fn new_xy(x: i32, y: i32) -> Self {
        Self { x, y, z: 0, w: 0 }
    }
}

impl Neighbors for Coord4D {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors_4d: Vec<Coord4D> = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        neighbors_4d.push(*self + Coord4D { x, y, z, w })
                    }
                }
            }
        }
        neighbors_4d
    }
}

type NeighborMap<T> = HashMap<T, usize>;

pub struct HashedGrid<T: std::hash::Hash + std::cmp::Eq + Debug + Copy + NewXY + Neighbors> {
    grid: HashSet<T>,
    neighbor_map: NeighborMap<T>,
}

impl<T> HashedGrid<T>
where
    T: std::hash::Hash + std::cmp::Eq + Debug + Copy + NewXY + Neighbors,
{
    fn from_2d_hash(data: &HashSet<Coord2D>) -> Self {
        let mut grid: HashSet<T> = HashSet::new();
        for point in data {
            grid.insert(T::new_xy(point.x, point.y));
        }
        let neighbor_map = HashedGrid::calculate_neighbors(&grid);
        Self { grid, neighbor_map }
    }

    fn calculate_neighbors(grid: &HashSet<T>) -> NeighborMap<T> {
        let mut neighbor_map: NeighborMap<T> = HashMap::new();
        for point in grid.iter() {
            let tmp_neighbors = point.neighbors();
            for neighbor in tmp_neighbors {
                *neighbor_map.entry(neighbor).or_insert(0) += 1;
            }
            // The map needs to contain existing points as well as their neighbors
            neighbor_map.entry(*point).or_insert(0);
        }
        neighbor_map
    }

    fn convolute(&mut self) {
        for (coord, num_neighbors) in &self.neighbor_map {
            if self.grid.contains(coord) {
                if *num_neighbors != 2 && *num_neighbors != 3 {
                    self.grid.remove(coord);
                }
            } else if *num_neighbors == 3 {
                self.grid.insert(*coord);
            }
        }
        self.neighbor_map = HashedGrid::calculate_neighbors(&self.grid);
    }

    fn cubes_active(&self) -> usize {
        self.grid.len()
    }
}

// fn print_3d_grid(grid: &HashSet<Coord3D>, z_plane: i32) {
//     let (min_x, max_x) = grid.iter().minmax_by_key(|item| item.x).into_option().unwrap();
//     let (min_y, max_y) = grid.iter().minmax_by_key(|item| item.y).into_option().unwrap();
//     println!("y: {}..{}, x: {}..{}", min_y.y, max_y.y, min_x.x, max_x.x);
//     for y in min_y.y..=max_y.y {
//         for x in min_x.x..=max_x.x {
//             if grid.contains(&Coord3D { x, y, z: z_plane }) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         print!("\n");
//     }
// }
//
// fn print_3d_neighbors(neighbor_map: &NeighborMap<Coord3D>, z_plane: i32) {
//     let (min_x, max_x) = neighbor_map.keys().minmax_by_key(|item| item.x).into_option().unwrap();
//     let (min_y, max_y) = neighbor_map.keys().minmax_by_key(|item| item.y).into_option().unwrap();
//     println!("y: {}..{}, x: {}..{}", min_y.y, max_y.y, min_x.x, max_x.x);
//     for y in min_y.y..=max_y.y {
//         for x in min_x.x..=max_x.x {
//             print!("{:02} ", neighbor_map.get(&Coord3D { x, y, z: z_plane }).unwrap_or(&0));
//         }
//         print!("\n");
//     }
// }

pub fn generator(input: &str) -> HashSet<Coord2D> {
    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut grid: HashSet<Coord2D> = HashSet::new();
    for c in input.chars() {
        match c {
            '#' => {
                grid.insert(Coord2D { x, y });
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => {
                x += 1;
            }
        }
    }
    grid
}

pub fn part_one(data: &HashSet<Coord2D>) -> usize {
    let mut field: HashedGrid<Coord3D> = HashedGrid::from_2d_hash(data);

    for _ in 1..=6 {
        field.convolute();
    }

    field.cubes_active()
}

pub fn part_two(data: &HashSet<Coord2D>) -> usize {
    let mut field: HashedGrid<Coord4D> = HashedGrid::from_2d_hash(data);

    for _ in 1..=6 {
        field.convolute();
    }

    field.cubes_active()
}
