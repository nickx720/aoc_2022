use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day9/input.txt")?;
    //let first_output = solution_one(input)?;
    let second_output = solution_two(input)?;
    // println!("The output is {first_output}");
    println!("The output is {second_output}");
    Ok(())
}

fn num_visited_by_tail(input: Vec<((isize, isize), isize)>, knots: usize) -> usize {
    std::iter::once((vec![(0, 0); knots + 1], input))
        .fold(
            std::collections::HashSet::from([(0, 0)]),
            |mut seen, (mut pos, motions)| {
                for ((dx, dy), steps) in motions {
                    (0..steps).for_each(|_| {
                        (0..3).for_each(|i| match i {
                            0 => pos[0] = (pos[0].0 + dx, pos[0].1 + dy),
                            1 => (1..pos.len()).for_each(|i| {
                                std::iter::once((pos[i - 1].0 - pos[i].0, pos[i - 1].1 - pos[i].1))
                                    .for_each(|(dx, dy)| {
                                        if dx.abs().max(dy.abs()) > 1 {
                                            pos[i] =
                                                (pos[i].0 + dx.signum(), pos[i].1 + dy.signum())
                                        }
                                    })
                            }),
                            _ => seen.extend(std::iter::once(pos[knots])),
                        })
                    })
                }
                seen
            },
        )
        .len()
}

fn solution_one(input: InputType) -> Result<usize, GlobalError> {
    let formatted_input = input
        .map(|item| {
            if let Ok(items) = item {
                items
            } else {
                panic!()
            }
        })
        .flat_map(|line| {
            line.split_once(' ')
                .and_then(|(d, n)| match (d, n.parse::<isize>().unwrap()) {
                    ("U", n) => Some(((0isize, 1isize), n)),
                    ("D", n) => Some(((0isize, -1isize), n)),
                    ("L", n) => Some(((-1isize, 0isize), n)),
                    ("R", n) => Some(((1isize, 0isize), n)),
                    _ => None,
                })
        })
        .collect::<Vec<((isize, isize), isize)>>();
    Ok(num_visited_by_tail(formatted_input, 1))
}

fn solution_two(input: InputType) -> Result<usize, GlobalError> {
    let formatted_input = input
        .map(|item| {
            if let Ok(items) = item {
                items
            } else {
                panic!()
            }
        })
        .flat_map(|line| {
            line.split_once(' ')
                .and_then(|(d, n)| match (d, n.parse::<isize>().unwrap()) {
                    ("U", n) => Some(((0isize, 1isize), n)),
                    ("D", n) => Some(((0isize, -1isize), n)),
                    ("L", n) => Some(((-1isize, 0isize), n)),
                    ("R", n) => Some(((1isize, 0isize), n)),
                    _ => None,
                })
        })
        .collect::<Vec<((isize, isize), isize)>>();
    Ok(num_visited_by_tail(formatted_input, 9))
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_day9_one() -> Result<(), GlobalError> {
    let input = readfile("assets/day9/sample.txt")?;
    let output = solution_one(input)?;
    assert_eq!(output, 13);
    Ok(())
}
#[test]
fn challenge_day9_two() -> Result<(), GlobalError> {
    let input = readfile("assets/day9/sample.txt")?;
    let output = solution_two(input)?;
    assert_eq!(output, 1);
    Ok(())
}
