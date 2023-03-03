use super::GlobalError;

fn to_snafu(decimal: i64) -> String {
    if decimal == 0 {
        return String::new();
    }

    let decimal_remainder = decimal % 5;
    let snafu_digit = ['0', '1', '2', '=', '-'][decimal_remainder as usize];
    let new_decimal = (decimal + 2) / 5;
    let mut snafu = to_snafu(new_decimal);
    snafu.push(snafu_digit);
    snafu
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |decimal, snafu_digit| {
        let decimal_digit = ['=', '-', '0', '1', '2']
            .into_iter()
            .position(|c| c == snafu_digit)
            .unwrap() as i64
            - 2;
        decimal * 5 + decimal_digit
    })
}

fn solution_one(path: &str) -> String {
    let input = std::fs::read_to_string(path).unwrap();
    let sum = input.lines().map(to_decimal).sum();
    to_snafu(sum)
}

pub fn run() -> Result<(), GlobalError> {
    let path = "assets/day25/input.txt";
    let output = solution_one(path);
    println!("The output for first part is {output}");
    Ok(())
}

#[test]
fn testing_challenge_one() -> Result<(), GlobalError> {
    let path = "assets/day25/sample.txt";
    let output = solution_one(path);
    assert_eq!("2=-1=0", output);
    Ok(())
}
#[test]
fn testing_challenge_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
