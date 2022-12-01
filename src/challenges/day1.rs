use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn challenge_one() -> Result<(u32)> {
    let file = File::open("assets/day1/input.txt")?;
    let reader = BufReader::new(file);

    let values = reader
        .lines()
        .map(|item| item.ok())
        .map(|item| {
            if let Some(item) = item {
                if item.is_empty() {
                    ".".to_string()
                } else {
                    let item = format!("{item}\n");
                    item
                }
            } else {
                "something went wrong".to_string()
            }
        })
        .collect::<String>();
    let mut values = values
        .split_terminator('.')
        .map(|item| {
            item.split_whitespace()
                .map(|item| item.parse::<u32>().ok())
                .filter(|item| item.is_some())
                .map(|item| item.unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    values.sort();

    Ok(*values.last().unwrap())
}

pub fn run() {
    let output = challenge_one().unwrap();
    println!("The output of first challenge is {output}");
}
