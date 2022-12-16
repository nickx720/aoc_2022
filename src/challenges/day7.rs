use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

pub fn challenge_one(input: InputType) -> Result<(), GlobalError> {
    Ok(())
}

pub fn challenge_two() -> Result<(), GlobalError> {
    Ok(())
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}

#[test]
fn challenge_day_7_test_one() -> Result<(), GlobalError> {
    let readfile = readfile("assets/day7/sample.txt")?;
    let output = challenge_one(readfile)?;
    assert_eq!(output, 5);
    Ok(())
}
#[test]
fn challenge_day_7_test_two() -> Result<(), GlobalError> {
    assert_eq!(23, 23);
    Ok(())
}
