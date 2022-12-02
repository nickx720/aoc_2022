use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

#[derive(Debug)]
pub enum GlobalError {
    IoError(Error),
}

impl Display for GlobalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalError::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl From<Error> for GlobalError {
    fn from(err: Error) -> Self {
        GlobalError::IoError(err)
    }
}

fn readfile(path: &str) -> Result<Lines<BufReader<File>>, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}

fn challenge_one(input: Lines<BufReader<File>>) -> i32 {
    let score: i32 = input
        .map(|item| {
            if let Ok(item) = item {
                item
            } else {
                "Something went wrong".to_string()
            }
        })
        .map(|item| {
            let bytes = item.as_bytes();
            let them = (bytes[0] - b'A') as i32;
            let me = (bytes[2] - b'X') as i32;

            let outcome = ((me - them).rem_euclid(3) + 1) % 3;
            outcome * 3 + me + 1
        })
        .sum();
    score
}
fn challenge_two(input: Lines<BufReader<File>>) -> i32 {
    let score: i32 = input
        .map(|item| {
            if let Ok(item) = item {
                item
            } else {
                "Something went wrong".to_string()
            }
        })
        .map(|item| {
            let bytes = item.as_bytes();
            let them = (bytes[0] - b'A') as i32;
            let outcome = (bytes[2] - b'X') as i32;

            let me = (them + outcome + 2) % 3;
            outcome * 3 + me + 1
        })
        .sum();
    score
}
pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day2/input.txt")?;
    //let output = challenge_one(input);
    let output_two = challenge_two(input);
    //println!("The first day challenge output is {output}");
    println!("The first day challenge output is {output_two}");
    Ok(())
}

#[test]
pub fn challenge_one_test() -> Result<(), GlobalError> {
    let input = readfile("assets/day2/sample.txt")?;
    let output = challenge_one(input);
    assert_eq!(output, 15);
    Ok(())
}
#[test]
pub fn challenge_two_test() -> Result<(), GlobalError> {
    let input = readfile("assets/day2/sample.txt")?;
    let output = challenge_two(input);
    assert_eq!(output, 12);
    Ok(())
}
