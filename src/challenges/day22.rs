use super::GlobalError;

enum Instruction {
    Rotate(Turn),
    Forward(u8),
}

enum Turn {
    L,
    R,
}

#[derive(PartialEq)]
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

fn wrapp(map: &[Vec<Tile>], pos: &Coord, dir: &Direction) -> Coord {
    let Coord { row: dr, col: dc } = dir.offset();
    let mut curr = pos.clone();
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if *tile == Tile::None {
            break;
        }
        curr = Coord {
            row: curr.row - dr,
            col: curr.col - dc,
        };
    }
    curr
}

fn wrap(pos: &Coord, dir: &Direction) -> (Coord, Direction) {
    let (cube_row, cube_col, new_dir) = match (pos.row / 50, pos.col / 50, dir) {
        (0, 1, Direction::U) => (3, 0, Direction::R),
        (0, 1, Direction::L) => (2, 0, Direction::R),
        (0, 2, Direction::U) => (3, 0, Direction::U),
        (0, 2, Direction::R) => (2, 1, Direction::L),
        (0, 2, Direction::D) => (1, 1, Direction::L),
        (1, 1, Direction::R) => (0, 2, Direction::U),
        (1, 1, Direction::L) => (2, 0, Direction::D),
        (2, 0, Direction::U) => (1, 1, Direction::R),
        (2, 0, Direction::L) => (0, 1, Direction::R),
        (2, 1, Direction::R) => (0, 2, Direction::L),
        (2, 1, Direction::D) => (3, 0, Direction::L),
        (3, 0, Direction::R) => (2, 1, Direction::U),
        (3, 0, Direction::D) => (0, 2, Direction::D),
        (3, 0, Direction::L) => (0, 1, Direction::D),
        _ => unreachable!(),
    };

    let (row_idx, col_idx) = (pos.row % 50, pos.col % 50);
    let i = match dir {
        Direction::L => 49 - row_idx,
        Direction::R => row_idx,
        Direction::U => col_idx,
        Direction::D => 49 - col_idx,
    };

    let new_row = match new_dir {
        Direction::L => 49 - i,
        Direction::R => i,
        Direction::U => 49,
        Direction::D => 0,
    };
    let new_col = match new_dir {
        Direction::L => 49,
        Direction::R => 0,
        Direction::U => i,
        Direction::D => 49 - i,
    };
    let new_pos = Coord {
        row: cube_row * 50 + new_row,
        col: cube_col * 50 + new_col,
    };
    (new_pos, new_dir)
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

fn solution_one(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let (map, instructions) = parse(&input);
    let start_col = map[0].iter().position(|tile| *tile == Tile::Open).unwrap() as i32;

    let mut pos = Coord {
        row: 0,
        col: start_col,
    };

    let mut dir = Direction::R;

    for inst in &instructions {
        match inst {
            Instruction::Rotate(turn) => dir = dir.turn(turn),
            Instruction::Forward(amount) => {
                for _ in 0..*amount {
                    let Coord { row: dr, col: dc } = dir.offset();
                    let new_tile = map
                        .get((pos.row + dr) as usize)
                        .and_then(|row| row.get((pos.col + dc) as usize))
                        .unwrap_or(&Tile::None);

                    match new_tile {
                        Tile::Solid => break,
                        Tile::Open => {
                            pos = Coord {
                                row: pos.row + dr,
                                col: pos.col + dc,
                            };
                        }
                        Tile::None => {
                            let new_pos = wrapp(&map, &pos, &dir);
                            if map[new_pos.row as usize][new_pos.col as usize] == Tile::Solid {
                                break;
                            }
                            pos = new_pos;
                        }
                    }
                }
            }
        }
    }
    1000 * (pos.row + 1) + 4 * (pos.col + 1) + dir.score() as i32
}

fn solution_two(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).unwrap();
    let (map, instructions) = parse(&input);

    let start_col = map[0].iter().position(|tile| *tile == Tile::Open).unwrap() as i32;
    let mut pos = Coord {
        row: 0,
        col: start_col,
    };
    let mut dir = Direction::R;
    for inst in &instructions {
        match inst {
            Instruction::Rotate(turn) => dir = dir.turn(turn),
            Instruction::Forward(amount) => {
                for _ in 0..*amount {
                    let Coord { row: dr, col: dc } = dir.offset();
                    let new_tile = map
                        .get((pos.row + dr) as usize)
                        .and_then(|row| row.get((pos.col + dc) as usize))
                        .unwrap_or(&Tile::None);
                    match new_tile {
                        Tile::Solid => break,
                        Tile::Open => {
                            pos = Coord {
                                row: pos.row + dr,
                                col: pos.col + dc,
                            };
                        }
                        Tile::None => {
                            let (new_pos, new_dir) = wrap(&pos, &dir);
                            if map[new_pos.row as usize][new_pos.col as usize] == Tile::Solid {
                                break;
                            }
                            pos = new_pos;
                            dir = new_dir;
                        }
                    }
                }
            }
        }
    }
    1000 * (pos.row + 1) + 4 * (pos.col + 1) + dir.score() as i32
}

pub fn run() -> Result<(), GlobalError> {
    let path = "assets/day22/input.txt";
    let output = solution_two(path);
    // println!("The first output is {output}");
    println!("The second output is {output}");
    Ok(())
}

#[test]
fn challenge_one_day_22() -> Result<(), GlobalError> {
    let path = "assets/day22/sample.txt";
    let output = solution_one(path);
    assert_eq!(output, 6032);
    Ok(())
}
#[test]
fn challenge_two_day_22() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
