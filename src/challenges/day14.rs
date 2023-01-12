use super::GlobalError;

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

struct Coord {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Vec<Vec<Coord>> {
    let input = std::fs::read_to_string(input).unwrap();
    input.lines().map(|line| {
        line.split(" -> ").map(|coords| {
            let (x,y) == coords.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Coord {x,y}
        }).collect()
    }).collect()

}

#[test]
fn challenge_one_day_13() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
#[test]
fn challenge_two_day_13() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
