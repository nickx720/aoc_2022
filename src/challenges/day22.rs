use super::GlobalError;

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

enum Turn {
    L,
    R,
}

enum Tile {
    Open,
    Solid,
    None,
}

#[derive(Clone)]
struct Coord {
    row: i32,
    col: i32,
}

enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn score(&self) -> usize {
        use Direction::*;
        match self {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        }
    }

    fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (L, Turn::L) => D,
            (L, Turn::R) => U,
            (R, Turn::L) => U,
            (R, Turn::R) => D,
            (U, Turn::L) => L,
            (U, Turn::R) => R,
            (D, Turn::L) => R,
            (D, Turn::R) => L,
        }
    }

    fn offset(&self) -> Coord {
        use Direction::*;
        match &self {
            L => Coord { row: 0, col: -1 },
            R => Coord { row: 0, col: 1 },
            U => Coord { row: -1, col: 0 },
            D => Coord { row: 1, col: 0 },
        }
    }
}

fn wrap(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> Coord {
    todo!()
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<Instruction>) {
    let (grid, moves) = input.trim_end().split_once("\n\n").unwrap();
    let mut instructions = Vec::new();
    let mut digits = Vec::new();
    for c in moves.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as u8;
            digits.push(digit);
        } else {
            let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
            digits.clear();
            instructions.push(Instruction::Forward(num));

            let turn = match c {
                'L' => Turn::L,
                'R' => Turn::R,
                _ => panic!("Invalid input"),
            };
            instructions.push(Instruction::Rotate(turn));
        }
    }
    let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
    instructions.push(Instruction::Forward(num));

    let mut map = Vec::new();
    for line in grid.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                ' ' => Tile::None,
                _ => panic!("invalid input"),
            };
            row.push(tile);
        }
        map.push(row);
    }
    (map, instructions)
}

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn challenge_one_day_22() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
#[test]
fn challenge_two_day_22() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
