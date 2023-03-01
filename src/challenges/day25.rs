use super::GlobalError;

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

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn testing_challenge_one() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
#[test]
fn testing_challenge_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
