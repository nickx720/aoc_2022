use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}
// https://github.com/IdrisTheDragon/Advent2022/blob/main/day-08/src/lib.rs#L30
// https://adventofcode.com/2022/day/8
fn solution_one(input: InputType) -> Result<u64, GlobalError> {
    Ok(2)
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_day8_one() -> Result<(), GlobalError> {
    let input = readfile("assets/day8/sample.txt")?;
    let output = solution_one(input)?;
    assert_eq!(output, 2);
    Ok(())
}
#[test]
fn challenge_day8_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
