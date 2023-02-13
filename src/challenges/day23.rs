use std::collections::HashSet;

use super::GlobalError;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn neighbours(&self) -> [Self; 8] {
        use Direction::*;
        let n = self.add_dir(&North);
        let ne = self.add_dir(&NorthEast);
        let e = self.add_dir(&East);
        let se = self.add_dir(&SouthEast);
        let s = self.add_dir(&South);
        let sw = self.add_dir(&SouthWest);
        let w = self.add_dir(&West);
        let nw = self.add_dir(&NorthWest);
        [n, ne, e, se, s, sw, w, nw]
    }
    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            North => Coord {
                row: self.row - 1,
                col: self.col,
            },
            NorthEast => Coord {
                row: self.row - 1,
                col: self.col + 1,
            },
            East => Coord {
                row: self.row,
                col: self.col + 1,
            },
            SouthEast => Coord {
                row: self.row + 1,
                col: self.col + 1,
            },
            South => Coord {
                row: self.row + 1,
                col: self.col,
            },
            SouthWest => Coord {
                row: self.row + 1,
                col: self.col - 1,
            },
            West => Coord {
                row: self.row,
                col: self.col - 1,
            },
            NorthWest => Coord {
                row: self.row - 1,
                col: self.col - 1,
            },
        }
    }
}

fn parse(input: &str) -> HashSet<Coord> {
    let mut elves = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }
    elves
}

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn check(&self, neighbours: &[bool; 8]) -> bool {
        todo!()
    }
}

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn challenge_23_part_one() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
#[test]
fn challenge_23_part_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
