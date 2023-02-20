use std::collections::HashMap;

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
