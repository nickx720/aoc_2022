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
