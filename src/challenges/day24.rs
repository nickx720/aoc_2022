use std::collections::{HashMap, HashSet};

use super::GlobalError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    pos: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        use Direction::*;
        let mut neighbours = Vec::new();
        if self.row > 0 {
            neighbours.push(self.add_dir(&Up));
        }
        if self.col < cols - 1 {
            neighbours.push(self.add_dir(&Right));
        }
        if self.row < rows - 1 {
            neighbours.push(self.add_dir(&Down));
        }
        if self.col > 0 {
            neighbours.push(self.add_dir(&Left));
        }
        neighbours
    }

    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

fn parse(input: &str) -> (HashMap<Coord, Tile>, usize, usize) {
    let mut map = HashMap::new();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = Coord { row, col };
            let tile = match c {
                '#' => Tile::Wall,
                '^' => Tile::Blizzard(Direction::Up),
                'v' => Tile::Blizzard(Direction::Down),
                '<' => Tile::Blizzard(Direction::Left),
                '>' => Tile::Blizzard(Direction::Right),
                _ => panic!("invalid input"),
            };
            map.insert(coord, tile);
        }
    }
    (map, rows, cols)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn blitz_maps(
    map: &HashMap<Coord, Tile>,
    rows: usize,
    cols: usize,
    max_time: usize,
) -> HashMap<usize, HashSet<Coord>> {
    let mut cache = HashMap::new();

    let mut blizzards: Vec<(Coord, Direction)> = map
        .iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Wall => None,
            Tile::Blizzard(dir) => Some((*pos, *dir)),
        })
        .collect();

    let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
    cache.insert(0, coords);

    for time in 1..max_time {
        for (coord, dir) in blizzards.iter_mut() {
            *coord = coord.add_dir(dir);
            match dir {
                Direction::Left => {
                    if coord.col == 0 {
                        coord.col = cols - 2;
                    }
                }

                Direction::Right => {
                    if coord.col == cols - 1 {
                        coord.col = 1;
                    }
                }
                Direction::Up => {
                    if coord.row == 0 {
                        coord.row = rows - 2;
                    }
                }
                Direction::Down => {
                    if coord.row == rows - 1 {
                        coord.row = 1;
                    }
                }
            }
        }
        let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
        cache.insert(time, coords);
    }
    cache
}

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn challenge_one_day_24() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}

#[test]
fn challenge_two_day_24() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
