use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day4/input.txt")?;
    //let output = challenge_one(input)?;
    let output_2 = challenge_two(input)?;
    // println!("The output of first one is {output:?}");
    println!("The output of second one is {output_2:?}");
    Ok(())
}

fn challenge_one(input: InputType) -> Result<usize, GlobalError> {
    let input: Vec<(usize, usize, usize, usize)> = input
        .map(|item| item.ok())
        .filter(|item| item.is_some())
        .map(|l| {
            let output = match l {
                Some(value) => value
                    .split(['-', ','])
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple::<(_, _, _, _)>(),
                None => unreachable!(),
            };
            output.unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
        .collect();
    Ok(input.len())
}

fn challenge_two(input: InputType) -> Result<usize, GlobalError> {
    let input: Vec<(usize, usize, usize, usize)> = input
        .map(|item| item.ok())
        .filter(|item| item.is_some())
        .map(|l| {
            let output = match l {
                Some(value) => value
                    .split(['-', ','])
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple::<(_, _, _, _)>(),
                None => unreachable!(),
            };
            output.unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1))
        .collect();
    Ok(input.len())
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}

#[test]
fn test_challenge_one() -> Result<(), GlobalError> {
    let input = readfile("assets/day4/sample.txt")?;
    let output = challenge_one(input)?;
    assert_eq!(output, 2);
    Ok(())
}
#[test]
fn test_challenge_two() -> Result<(), GlobalError> {
    let input = readfile("assets/day4/sample.txt")?;
    let output = challenge_two(input)?;
    assert_eq!(output, 4);
    Ok(())
}
