use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::Lines,
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let first_input = include_str!("../../assets/day7/input.txt").lines();
    //    let first_output = challenge_one(first_input)?;
    let second_output = challenge_two(first_input)?;
    println!("The first output is {second_output}");
    Ok(())
}

pub fn challenge_one(mut input: Lines) -> Result<u64, GlobalError> {
    let output = parse(&mut input)
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    Ok(output)
}

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", s]) if *s != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

pub fn challenge_two(mut input: Lines) -> Result<u64, GlobalError> {
    let mut sizes = parse(&mut input);
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    Ok(sizes.into_iter().find(|&s| s >= missing).unwrap())
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}

#[test]
fn challenge_day_7_test_one() -> Result<(), GlobalError> {
    let input = include_str!("../../assets/day7/sample.txt");
    let output = challenge_one(input.lines())?;
    assert_eq!(output, 95437);
    Ok(())
}
#[test]
fn challenge_day_7_test_two() -> Result<(), GlobalError> {
    let input = include_str!("../../assets/day7/sample.txt");
    let output = challenge_two(input.lines())?;
    assert_eq!(24933642, output);
    Ok(())
}
