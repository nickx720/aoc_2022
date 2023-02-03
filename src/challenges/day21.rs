use std::collections::HashMap;

use super::GlobalError;

enum Monkey<'a> {
    Num(i64),
    Calculated(Operator, &'a str, &'a str),
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    input
        .lines()
        .map(|line| {
            let (name, right) = line.split_once(": ").unwrap();
            let monkey = match right.parse() {
                Ok(n) => Monkey::Num(n),
                Err(_) => {
                    let mut iter = right.split_whitespace();
                    let lhs = iter.next().unwrap();
                    let operator = match iter.next().unwrap() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        "*" => Operator::Mul,
                        "/" => Operator::Div,
                        _ => panic!("Invalid math operator"),
                    };
                    let rhs = iter.next().unwrap();
                    Monkey::Calculated(operator, lhs, rhs)
                }
            };
            (name, monkey)
        })
        .collect()
}

fn calc_name(name: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Calculated(operator, lhs, rhs) => {
            let lhs_num = calc_name(lhs, monkeys);
            let rhs_num = calc_name(rhs, monkeys);
            match operator {
                Operator::Add => lhs_num + rhs_num,
                Operator::Sub => lhs_num - rhs_num,
                Operator::Mul => lhs_num * rhs_num,
                &Operator::Div => lhs_num / rhs_num,
            }
        }
    }
}

fn calc_human(name: &str, value: i64, monkeys: HashMap<&str, Monkey>) -> i64 {
    if name == "humn" {
        return value;
    }
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Calculated(operator, lhs, rhs) => {
            let (new_name, new_value) = if depends_on_human(lhs, &monkeys) {
                let rhs_num = calc_name(rhs, &monkeys);
                let new_value = match operator {
                    Operator::Add => value - rhs_num,
                    Operator::Sub => value + rhs_num,
                    Operator::Mul => value / rhs_num,
                    Operator::Div => value * rhs_num,
                };
                (lhs, new_value)
            } else {
                let lhs_num = calc_name(lhs, &monkeys);
                let new_value = match operator {
                    Operator::Add => value - lhs_num,
                    Operator::Sub => lhs_num - value,
                    Operator::Mul => value / lhs_num,
                    Operator::Div => lhs_num / value,
                };
                (rhs, new_value)
            };
            calc_human(new_name, new_value, monkeys)
        }
    }
}

fn depends_on_human(name: &str, monkeys: &HashMap<&str, Monkey>) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkeys[name] {
        Monkey::Num(_) => false,
        Monkey::Calculated(_, lhs, rhs) => {
            depends_on_human(lhs, monkeys) || depends_on_human(rhs, monkeys)
        }
    }
}

fn solution_one(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let monkeys = parse(&input);
    calc_name("root", &monkeys)
}
fn solution_two(path: &str) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let monkeys = parse(&input);
    let Monkey::Calculated(_, lhs, rhs) = &monkeys["root"] else {
        panic!("root has to be calculated monkey");
    };

    let (name, value) = if depends_on_human(lhs, &monkeys) {
        let rhs_num = calc_name(rhs, &monkeys);
        (lhs, rhs_num)
    } else {
        let lhs_num = calc_name(lhs, &monkeys);
        (rhs, lhs_num)
    };
    calc_human(name, value, monkeys)
}
pub fn run() -> Result<(), GlobalError> {
    let path = "assets/day21/input.txt";
    //let output = solution_one(path);
    let output = solution_two(path);
    println!("The output is {output}");
    Ok(())
}

#[test]
fn challenge_one_day_21() -> Result<(), GlobalError> {
    let path = "assets/day21/sample.txt";
    let output = solution_one(path);
    assert_eq!(output, 152);
    Ok(())
}
#[test]
fn challenge_two_day_21() -> Result<(), GlobalError> {
    let path = "assets/day21/sample.txt";
    let output = solution_two(path);
    assert_eq!(output, 301);
    Ok(())
}
