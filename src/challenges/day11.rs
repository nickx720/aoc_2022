use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_one_day11() -> Result<(), GlobalError> {
    assert_eq!(10605, 10605);
    Ok(())
}
#[test]
fn challenge_two_day11() -> Result<(), GlobalError> {
    assert_eq!(10605, 10605);
    Ok(())
}
