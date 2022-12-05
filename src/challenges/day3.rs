use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day3/input.txt")?;
    //let output = challenge_one(input)?;
    let output = challenge_two(input)?;
    println!("The second output is {output}");
    Ok(())
}

fn compartment_to_bitmap(compartment: &str) -> u64 {
    compartment.chars().fold(0, |a, char| {
        a | (1
            << match char {
                'a'..='z' => char as u64 - 97,
                'A'..='Z' => char as u64 - 39,
                _ => unreachable!(),
            })
    })
}

fn challenge_one(input: InputType) -> Result<u64, GlobalError> {
    let mut priority_sum = 0;
    input
        .map(|item| {
            if let Ok(value) = item {
                value
            } else {
                "Something went wrong".to_string()
            }
        })
        .for_each(|n| {
            let (compartment_a, compartment_b) = n.split_at(n.len() / 2);
            let compartment_a = compartment_to_bitmap(compartment_a);
            let compartment_b = compartment_to_bitmap(compartment_b);

            let duplicate = ((compartment_a & compartment_b) as f64).log2() as u64;

            priority_sum += duplicate + 1;
        });
    Ok(priority_sum)
}
fn challenge_two(input: InputType) -> Result<u32, GlobalError> {
    fn priority(item: char) -> u32 {
        if ('a'..='z').contains(&item) {
            item as u32 - 96
        } else if ('A'..='Z').contains(&item) {
            item as u32 - 38
        } else {
            0
        }
    }
    let input = input
        .map(|item| item.unwrap())
        .map(|item| item.split_whitespace().collect::<String>())
        .collect::<Vec<String>>();
    let input: Vec<HashSet<char>> = input
        .iter()
        .map(|sack| sack.chars().collect::<HashSet<char>>())
        .collect();
    let input: u32 = input
        .chunks(3)
        .map(|chunk| {
            for candidate in chunk[0].intersection(&chunk[1]) {
                if chunk[2].contains(candidate) {
                    return priority(*candidate);
                }
            }
            0
        })
        .sum();
    Ok(input)
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_day_3_one() -> Result<(), GlobalError> {
    let input = readfile("assets/day3/sample.txt")?;
    let output = challenge_one(input)?;
    assert_eq!(output, 157);
    Ok(())
}

#[test]
fn challenge_day_3_two() -> Result<(), GlobalError> {
    let input = readfile("assets/day3/sample.txt")?;
    let output = challenge_two(input)?;
    assert_eq!(output, 70);
    Ok(())
}
