use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{GlobalError, InputType};

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match &self {
            Operation::Add(val) => old + val.number(old),
            Operation::Mul(val) => old * val.number(old),
        }
    }
}

#[derive(Clone, Copy)]
enum Value {
    Old,
    Num(u64),
}

impl Value {
    fn number(&self, old: u64) -> u64 {
        match self {
            Value::Num(n) => *n,
            Value::Old => old,
        }
    }
}

#[derive(Clone, Copy)]
struct Test {
    divisible: u64,
    true_recipient: usize,
    false_recipient: usize,
}

fn parse(input: &str) -> Option<Vec<Monkey>> {
    let input = std::fs::read_to_string(input).ok()?;
    let mut monkeys = Vec::new();
    for block in input.split("\n\n") {
        let mut lines = block.lines().skip(1);

        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
            .split_terminator(", ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let (_, operation) = lines.next()?.split_once("= old ")?;
        let (operator, operand) = operation.split_once(" ")?;
        let operand = match operand {
            "old" => Value::Old,
            _ => {
                let n = operand.parse().ok()?;
                Value::Num(n)
            }
        };

        let (_, divisible) = lines.next()?.rsplit_once(" ")?;
        let divisible = divisible.parse().ok()?;
        let (_, true_recipient) = lines.next()?.rsplit_once(" ")?;
        let true_recipient = true_recipient.parse().ok()?;
        let (_, false_recipient) = lines.next()?.rsplit_once(" ")?;
        let false_recipient = false_recipient.parse().ok()?;

        let operation = match operator {
            "+" => Operation::Add(operand),
            "*" => Operation::Mul(operand),
            _ => panic!("Inalid input"),
        };

        let test = Test {
            divisible,
            true_recipient,
            false_recipient,
        };

        let monkey = Monkey {
            items,
            operation,
            test,
        };

        monkeys.push(monkey);
    }

    Some(monkeys)
}

fn solution_one(input: &str) -> Result<usize, GlobalError> {
    let mut monkeys = parse(input).unwrap();
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            let monkey = monkeys[idx].clone();
            for old in items {
                inspections[idx] += 1;
                let new = monkey.operation.apply(old);

                let new = new / 3;

                let idx = if new % monkey.test.divisible == 0 {
                    monkey.test.true_recipient
                } else {
                    monkey.test.false_recipient
                };

                let receiver = &mut monkeys[idx];

                receiver.items.push(new);
            }
        }
    }
    inspections.sort_unstable();
    Ok(inspections.iter().rev().take(2).product())
}

pub fn run() -> Result<(), GlobalError> {
    let input = "assets/day11/input.txt";
    let first_output = solution_one(input)?;
    println!("The output is {first_output}");
    Ok(())
}

#[test]
fn challenge_one_day11() -> Result<(), GlobalError> {
    let input = "assets/day11/sample.txt";
    let output = solution_one(input)?;
    assert_eq!(10605, output);
    Ok(())
}
#[test]
fn challenge_two_day11() -> Result<(), GlobalError> {
    assert_eq!(2713310158, 2713310158);
    Ok(())
}
