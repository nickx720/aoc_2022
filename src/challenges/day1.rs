use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn challenge_one() -> Result<()> {
    let file = File::open("assets/day1/sample.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

pub fn run() {
    challenge_one().unwrap();
}
