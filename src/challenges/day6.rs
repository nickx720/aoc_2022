use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day6/input.txt")?;
    //let output = challenge_one(input)?;
    let output = challenge_two(input)?;
    println!("The output for second one is {output}");
    Ok(())
}

fn find_marker(input: &str, n: usize) -> usize {
    let input = input.trim().chars().collect_vec();
    input
        .windows(n)
        .enumerate()
        .filter(|(_, window)| window.into_iter().all_unique())
        .map(|(i, _)| i + n)
        .next()
        .unwrap()
}

fn challenge_two(input: InputType) -> Result<usize, GlobalError> {
    let input = input
        .map(|item| {
            if let Ok(value) = item {
                value
            } else {
                "Something went horribly wrong".to_string()
            }
        })
        .collect::<String>();
    Ok(find_marker(&input, 14))
}
fn challenge_one(input: InputType) -> Result<usize, GlobalError> {
    let input = input
        .map(|item| {
            if let Ok(value) = item {
                value
            } else {
                "Something went horribly wrong".to_string()
            }
        })
        .collect::<String>();
    Ok(find_marker(&input, 4))
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_day_6_test_one() -> Result<(), GlobalError> {
    let readfile = readfile("assets/day6/sample.txt")?;
    let output = challenge_one(readfile)?;
    assert_eq!(output, 5);
    Ok(())
}
#[test]
fn challenge_day_6_test_two() -> Result<(), GlobalError> {
    let readfile = readfile("assets/day6/sample.txt")?;
    let output = challenge_two(readfile)?;
    assert_eq!(output, 23);
    Ok(())
}
