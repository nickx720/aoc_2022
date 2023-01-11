use itertools::Itertools;

use super::GlobalError;

pub fn run() -> Result<(), GlobalError> {
    let input = "assets/day13/input.txt";
    //  let output_first = solution_1(input);
    //  println!("The output is {output_first}");
    let output_second = solution_2(input);
    println!("The output is {output_second}");
    Ok(())
}

#[derive(PartialEq, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

fn parse_num(list: &[char]) -> (Packet, &[char]) {
    let mut i = 0;
    while i < list.len() && list[i].is_ascii_digit() {
        i += 1;
    }

    let mut num = 0;
    let mut offset = 1;
    for c in list[..i].iter().rev() {
        num += (*c as u8 - b'0') * offset;
        offset *= 10;
    }

    (Packet::Num(num), &list[i..])
}

fn parse_list(list: &[char]) -> (Packet, &[char]) {
    let mut list = &list[1..];
    let mut packets = Vec::new();

    loop {
        match list[0] {
            ']' => break,
            ',' => list = &list[1..],
            '[' => {
                let (packet, rest) = parse_list(list);
                packets.push(packet);
                list = rest;
            }
            _ => {
                let (n, rest) = parse_num(list);
                packets.push(n);
                list = rest;
            }
        }
    }
    (Packet::List(packets), &list[1..])
}

fn parse_packet(s: &str) -> Packet {
    let chars: Vec<_> = s.chars().collect();

    let (packet, _) = parse_list(&chars);
    packet
}

fn parse(input: &str) -> Vec<[Packet; 2]> {
    let input = std::fs::read_to_string(input).unwrap();

    input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();

            [parse_packet(left), parse_packet(right)]
        })
        .collect()
}

fn solution_1(input: &str) -> usize {
    let pairs = parse(input);
    pairs
        .iter()
        .positions(|[left, right]| left < right)
        .map(|idx| idx + 1)
        .sum()
}

fn solution_2(input: &str) -> usize {
    let pairs = parse(input);
    let mut packets: Vec<_> = pairs.iter().flatten().collect();

    let divider_1 = parse_packet("[[2]]");
    let divider_2 = parse_packet("[[6]]");

    packets.push(&divider_1);
    packets.push(&divider_2);

    packets.sort_unstable();

    packets
        .into_iter()
        .positions(|packet| packet == &divider_1 || packet == &divider_2)
        .map(|idx| idx + 1)
        .product()
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn solution_one_test_day_13() -> Result<(), GlobalError> {
    let input = "assets/day13/sample.txt";
    let ouput = solution_1(input);
    assert_eq!(13, ouput);
    Ok(())
}
#[test]
fn solution_two_test_day_13() -> Result<(), GlobalError> {
    let input = "assets/day13/sample.txt";
    let ouput = solution_2(input);
    assert_eq!(140, 140);
    Ok(())
}
