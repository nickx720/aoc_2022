use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

pub fn run() -> Result<(), GlobalError> {
    let input = readfile("assets/day10/input.txt")?;
    //    let first_input = solution_one(input)?;
    //    println!("The first output is {first_input}");
    let second_input = solution_two(input)?;
    println!("The second output is {second_input}");
    Ok(())
}

fn solution_one(input: InputType) -> Result<isize, GlobalError> {
    Ok(solve(input).2)
}
fn solution_two(input: InputType) -> Result<String, GlobalError> {
    Ok(reading_the_values(solve(input).3))
}

fn solve(input: InputType) -> (isize, isize, isize, Vec<Vec<char>>) {
    let formatted_input = input.map(|item| {
        if let Ok(items) = item {
            items
        } else {
            panic!()
        }
    });
    formatted_input
        .flat_map(|line| match line.splitn(2, ' ').collect::<Vec<_>>()[..] {
            ["addx", n] => Some((2, n.parse().unwrap())),
            ["noop"] => Some((1, 0)),
            _ => None,
        })
        .fold(
            (0, 1, 0, vec![vec![' '; 40]; 6]),
            |(mut cycle, mut x, mut strengths, mut crt), (ticks, add)| {
                for i in (0..ticks).rev() {
                    (0..4).for_each(|lol| match lol {
                        0 => {
                            if x.abs_diff(cycle % 40) <= 1 {
                                crt[(cycle / 40) as usize][(cycle % 40) as usize] = '#'
                            }
                        }
                        1 => cycle += 1,
                        2 => {
                            if (cycle % 40) - 20 == 0 {
                                strengths += cycle * x
                            }
                        }
                        _ => {
                            if i == 0 {
                                x += add
                            }
                        }
                    })
                }
                (cycle, x, strengths, crt)
            },
        )
}

fn reading_the_values(display: Vec<Vec<char>>) -> String {
    std::iter::once(std::collections::HashMap::from([
        (" ##  #  # #  # #### #  # #  # ", 'A'),
        ("###  #  # ###  #  # #  # ###  ", 'B'),
        (" ##  #  # #    #    #  #  ##  ", 'C'),
        ("#### #    ###  #    #    #### ", 'E'),
        ("#### #    ###  #    #    #    ", 'F'),
        (" ##  #  # #    # ## #  #  ### ", 'G'),
        ("#  # #  # #### #  # #  # #  # ", 'H'),
        (" ###   #    #    #    #   ### ", 'I'),
        ("  ##    #    #    # #  #  ##  ", 'J'),
        ("#  # # #  ##   # #  # #  #  # ", 'K'),
        ("#    #    #    #    #    #### ", 'L'),
        (" ##  #  # #  # #  # #  #  ##  ", 'O'),
        ("###  #  # #  # ###  #    #    ", 'P'),
        ("###  #  # #  # ###  # #  #  # ", 'R'),
        (" ### #    #     ##     # ###  ", 'S'),
        ("#  # #  # #  # #  # #  #  ##  ", 'U'),
        ("#    #     # #   #    #    #  ", 'Y'),
        ("####    #   #   #   #    #### ", 'Z'),
    ]))
    .fold(String::new(), |a, dict| {
        let output = display
            .iter()
            .fold(vec![a; 8], |mut a, l| {
                for (i, c) in l.chunks_exact(5).enumerate() {
                    a[i] += &c.iter().collect::<String>()
                }
                a
            })
            .iter()
            .flat_map(|c| dict.get(c as &str))
            .collect();
        output
    })
}

fn readfile(path: &str) -> Result<InputType, GlobalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let output = reader.lines();
    Ok(output)
}
#[test]
fn challenge_day10_one() -> Result<(), GlobalError> {
    let input = readfile("assets/day10/sample.txt")?;
    let output = solution_one(input)?;
    assert_eq!(output, 13140);
    Ok(())
}
#[test]
fn challenge_day9_two() -> Result<(), GlobalError> {
    let input = readfile("assets/day10/sample.txt")?;
    let output = solution_two(input)?;
    assert_eq!(output, "".to_string());
    Ok(())
}
