use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error},
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

fn readfile(path: &str) -> Result<String, GlobalError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(String::new())
}

fn challenge_one(input: String) -> u32 {
    15
}
pub fn run() {}

#[test]
pub fn challenge_one_test() -> Result<(), GlobalError> {
    let input = readfile("assets/day2/sample.txt")?;
    let output = challenge_one(input);
    assert_eq!(output, 15);
    Ok(())
}
