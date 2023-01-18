use std::{collections::HashMap, fmt::Display};

use super::GlobalError;

enum Jet {
    Left,
    Right,
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("invalid input, {}", c),
        })
        .collect()
}

#[derive(Debug, PartialEq, Default)]
struct Coord {
    x: usize,
    y: usize,
}

const WIDTH: usize = 7;
const PIECES: [&[Coord]; 5] = [
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 3, y: 0 },
    ],
    &[
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 0 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
        Coord { x: 2, y: 1 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: 2 },
        Coord { x: 0, y: 3 },
    ],
    &[
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ],
];

#[derive(Default)]
struct State {
    jet_count: usize,
    piece_count: usize,
    top: usize,
    map: Vec<[bool; WIDTH]>,
    curr: Coord,
    seen: HashMap<(usize, usize), (usize, usize, usize)>,
    added_by_repeats: usize,
}

impl State {
    fn is_valid(&mut self, new_curr: &Coord, piece: &[Coord]) -> bool {
        piece.iter().all(|offset| {
            let x = new_curr.x + offset.x;
            let y = new_curr.y + offset.y;
            while self.map.len() <= y {
                self.map.push([false; WIDTH]);
            }
            x < WIDTH && !self.map[y][x]
        })
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = PIECES[self.piece_count % PIECES.len()];
        let mut print: Vec<Vec<_>> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|rock| if *rock { '#' } else { '.' })
                    .collect()
            })
            .collect();

        let mut local_top = self.top;
        for offset in piece {
            let x = self.curr.x + offset.x;
            let y = self.curr.y + offset.y;
            while print.len() <= y {
                print.push(vec!['.'; WIDTH]);
            }
            print[y][x] = '@';
            local_top = local_top.max(y + 1);
        }
        for row in (0..local_top).rev() {
            let mut row_str = String::from('|');
            for col in 0..7 {
                row_str.push(print[row][col]);
            }
            row_str.push('|');
            row_str.push('\n');
            write!(f, "{row_str}")?;
        }
        writeln!(f, "+{}+", "-".repeat(WIDTH))
    }
}

fn solution_one(input: &str) -> usize {
    let target = 2022;
    let input = std::fs::read_to_string(input).unwrap();
    let jets = parse(&input);
    let mut state = State::default();

    while state.piece_count != target {
        let piece = PIECES[state.piece_count % PIECES.len()];
        state.curr.x = 2;
        state.curr.y = state.top + 3;

        loop {
            let jet = &jets[state.jet_count % jets.len()];
            let new_curr = match jet {
                Jet::Left => Coord {
                    x: state.curr.x.saturating_sub(1),
                    y: state.curr.y,
                },
                Jet::Right => Coord {
                    x: state.curr.x + 1,
                    y: state.curr.y,
                },
            };
            if state.is_valid(&new_curr, piece) {
                state.curr = new_curr;
            }
            state.jet_count += 1;
            let new_curr = Coord {
                x: state.curr.x,
                y: state.curr.y.saturating_sub(1),
            };
            if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                break;
            }
            state.curr = new_curr;
        }
        for offset in piece {
            let x = state.curr.x + offset.x;
            let y = state.curr.y + offset.y;
            while state.map.len() <= y {
                state.map.push([false; WIDTH]);
            }
            state.map[y][x] = true;
            state.top = state.top.max(y + 1);
        }
        state.piece_count += 1;
    }
    state.top
}

fn solution_two(input: &str) -> usize {
    let target = 1_000_000_000_000;
    let input = std::fs::read_to_string(input).unwrap();
    let jets = parse(&input);
    let mut state = State::default();
    while state.piece_count != target {
        let piece = PIECES[state.piece_count % PIECES.len()];
        state.curr.x = 2;
        state.curr.y = state.top + 3;

        loop {
            let jet = &jets[state.jet_count % jets.len()];
            let new_curr = match jet {
                Jet::Left => Coord {
                    x: state.curr.x.saturating_sub(1),
                    y: state.curr.y,
                },
                Jet::Right => Coord {
                    x: state.curr.x + 1,
                    y: state.curr.y,
                },
            };
            if state.is_valid(&new_curr, piece) {
                state.curr = new_curr;
            }
            state.jet_count += 1;

            let new_curr = Coord {
                x: state.curr.x,
                y: state.curr.y.saturating_sub(1),
            };
            if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                break;
            }
            state.curr = new_curr;
        }

        for offset in piece {
            let x = state.curr.x + offset.x;
            let y = state.curr.y + offset.y;
            while state.map.len() <= y {
                state.map.push([false; WIDTH]);
            }
            state.map[y][x] = true;
            state.top = state.top.max(y + 1);
        }

        if state.added_by_repeats == 0 {
            let key = (
                state.piece_count % PIECES.len(),
                state.jet_count % jets.len(),
            );
            if let Some((2, old_piece_count, old_top)) = state.seen.get(&key) {
                let delta_top = state.top - old_top;
                let delta_piece_count = state.piece_count - old_piece_count;
                let repeats = (target - state.piece_count) / delta_piece_count;
                state.added_by_repeats += repeats * delta_top;
                state.piece_count += repeats * delta_piece_count;
            }

            state
                .seen
                .entry(key)
                .and_modify(|(amnt, old_piece_count, old_top)| {
                    *amnt += 1;
                    *old_piece_count = state.piece_count;
                    *old_top = state.top;
                })
                .or_insert((1, state.piece_count, state.top));
        }
        state.piece_count += 1;
    }
    state.top + state.added_by_repeats
}
pub fn run() -> Result<(), GlobalError> {
    let input = "assets/day17/input.txt";
    let output = solution_two(input);
    println!("The output is {output} ");
    Ok(())
}

#[test]
fn challenge_one_day_17() -> Result<(), GlobalError> {
    let input = "assets/day17/sample.txt";
    let output = solution_one(input);
    assert_eq!(output, 3068);
    Ok(())
}
#[test]
fn challenge_two_day_17() -> Result<(), GlobalError> {
    let input = "assets/day17/sample.txt";
    let output = solution_two(input);
    assert_eq!(output, 1514285714288);
    Ok(())
}
