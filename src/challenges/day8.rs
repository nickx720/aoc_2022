use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day8/input.txt")?;
    let first_output = solution_one(input)?;
    println!("The first output is {first_output}");
    Ok(())
}
// https://github.com/IdrisTheDragon/Advent2022/blob/main/day-08/src/lib.rs#L30
// https://adventofcode.com/2022/day/8
fn solution_one(input: InputType) -> Result<usize, GlobalError> {
    let data = input
        .map(|l| if let Ok(value) = l { value } else { panic!() })
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).ok_or("Something is wrong"))
                .collect()
        })
        .collect::<Result<Vec<Vec<u32>>, &str>>();

    let (mut count, mut best_view) = (0, 0);

    if let Ok(data) = data {
        count += (data.len() - 2) * 2;
        count += data[0].len() * 2;

        for row in 1..data.len() - 1 {
            for col in 1..data[0].len() - 1 {
                let r = data.get(row).ok_or("missing  r").unwrap();
                let h = r.get(col).ok_or("missing v").unwrap();

                if r[0..col].iter().max().ok_or("no max").unwrap() < h
                    || r[col + 1..r.len()].iter().max().ok_or("no max").unwrap() < h
                    || data[0..row]
                        .iter()
                        .map(|v| v[col])
                        .max()
                        .ok_or("no max")
                        .unwrap()
                        < *h
                    || data[row + 1..data.len()]
                        .iter()
                        .map(|v| v[col])
                        .max()
                        .ok_or("no max")
                        .unwrap()
                        < *h
                {
                    count += 1;
                }
            }
        }
    } else {
        panic!()
    }

    Ok(count)
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
    assert_eq!(output, 21);
    Ok(())
}
#[test]
fn challenge_day8_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
