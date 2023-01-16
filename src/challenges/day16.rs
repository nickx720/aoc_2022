use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

use super::GlobalError;

struct Valve<'a> {
    flow: u32,
    neighbours: HashSet<&'a str>,
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();
    pq.push(Node {
        cost: 0,
        curr: from,
    });
    seen.insert(from);

    while let Some(Node { cost, curr }) = pq.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].neighbours.iter() {
            if seen.insert(neighbour) {
                pq.push(Node {
                    cost: cost + 1,
                    curr: neighbour,
                });
            }
        }
    }
    u32::MAX
}

fn min_distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));
            let dist = min_cost(name1, name2, map);
            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);
            acc
        })
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    input
        .lines()
        .map(|line| {
            let (valve, neighbours) = line.split_once("; ").unwrap();
            let valve = valve.strip_prefix("Valve ").unwrap();
            let (name, flow) = valve.split_once(" has flow rate=").unwrap();
            let flow = flow.parse().unwrap();
            let neighbours = neighbours
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| neighbours.strip_prefix("tunnel leads to valve "))
                .unwrap();
            let neighbours = neighbours.split_terminator(", ").collect();
            (name, Valve { flow, neighbours })
        })
        .collect()
}

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn day16_day_one() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
#[test]
fn day16_day_two() -> Result<(), GlobalError> {
    assert_eq!(2, 2);
    Ok(())
}
