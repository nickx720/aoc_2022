use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Error, Lines},
};

//mod day1;
//mod day2;
//mod day3;
//mod day4;
//mod day5;
//mod day6;
// mod day7;
//mod day8;
//mod day9;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
mod day22;
pub fn run() {
    day22::run().unwrap();
}

pub type InputType = Lines<BufReader<File>>;
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
