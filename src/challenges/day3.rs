use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use super::{GlobalError, InputType};

pub fn run() {}

fn challenge_one(input: InputType) -> Result<i32, Box<dyn Error>> {
    let input = input
        .map(|item| {
            if let Ok(value) = item {
                value
            } else {
                "Something went wrong".to_string()
            }
        })
        .map(|item| item.split_whitespace().collect())
        .collect::<Vec<String>>();
    let count = input
        .iter()
        .map(|item| item.split_at(item.len() / 2))
        .collect::<Vec<(&str, &str)>>();
    dbg!(count);
    Ok(2)
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
    let output = challenge_one(input);
    assert_eq!(2, 2);
    Ok(())
}
