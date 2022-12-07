use std::{collections::HashMap, fs::read_to_string};

use super::GlobalError;

pub fn run() -> Result<(), GlobalError> {
    let first_input = part_one("assets/day5/input.txt");
    println!("The first one output is {first_input}");
    let second_outout = part_two("assets/day5/input.txt");
    println!("The first one output is {second_outout}");
    Ok(())
}

type Stack = Vec<Vec<char>>;
type Moves = Vec<Vec<usize>>;

fn parse_input(path: &str) -> (Stack, Moves) {
    let str = read_to_string(path).unwrap();

    let (raw_stacks, raw_moves) = str.split_once("\n\n").unwrap();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for line in raw_stacks.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                stacks
                    .entry(i)
                    .and_modify(|vec| vec.push(c))
                    .or_insert(vec![c]);
            }
        }
    }

    let mut moves: Moves = vec![];
    for line in raw_moves.lines() {
        let splits: Vec<&str> = line.split(" ").collect();
        let set: Vec<usize> = splits.iter().filter_map(|elem| elem.parse().ok()).collect();
        moves.push(set);
    }
    let mut sorted: Vec<_> = stacks.drain().collect();
    sorted.sort();

    let stacks: Stack = sorted
        .into_iter()
        .map(|(_, st)| st.into_iter().rev().collect())
        .collect();

    (stacks, moves)
}

fn string_from_tops(stacks: Stack) -> String {
    stacks.into_iter().fold(String::new(), |mut str, mut vec| {
        str.push(vec.pop().unwrap());
        str
    })
}

fn part_one(path: &str) -> String {
    let (mut stacks, moves) = parse_input(path);

    for m in moves {
        let (count, from, to) = (m[0], m[1] - 1, m[2] - 1);
        for _ in 0..count {
            let v = stacks[from].pop().expect("Failed to pop");
            stacks[to].push(v);
        }
    }

    string_from_tops(stacks)
}

fn part_two(path: &str) -> String {
    let (mut stacks, moves) = parse_input(path);
    for m in moves {
        let (count, from, to) = (m[0], m[1] - 1, m[2] - 1);

        let len = stacks[from].len();
        let mut v: Vec<_> = stacks[from].drain(len - count..).collect();

        stacks[to].append(&mut v);
    }

    string_from_tops(stacks)
}
#[test]
fn test_challenge_one_day5() -> Result<(), GlobalError> {
    let output = part_one("assets/day5/sample.txt");
    assert_eq!(output, String::from("CMZ"));
    Ok(())
}
#[test]
fn test_challenge_two_day5() -> Result<(), GlobalError> {
    let output = part_two("assets/day5/sample.txt");
    assert_eq!(output, String::from("MCD"));
    Ok(())
}
