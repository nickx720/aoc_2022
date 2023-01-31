use std::collections::VecDeque;

use super::GlobalError;

struct State {
    inventory: [u16; 4],
    bots: [u16; 4],
    elapsed: u16,
}

fn max_geodes(blueprint: &[[u16; 4]; 4]) -> u16 {
    let max_time = 24;
    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State {
        inventory,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        for i in 0..blueprint.len() {
            let costs = &blueprint[i];
            let wait_time = (0..3)
                .map(|idx| match costs[idx] {
                    cost if cost <= inventory[idx] => 0,
                    _ if bots[idx] == 0 => max_time + 1,
                    _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                })
                .max()
                .unwrap();
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }
            let mut new_bots = bots;
            new_bots[i] += 1;

            q.push_back(State {
                inventory: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }
        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }
    max_geodes
}

fn parse(input: &str) -> Vec<[[u16; 4]; 4]> {
    let input = std::fs::read_to_string(input).unwrap();
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];

        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];

        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];
        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }
    blueprints
}

fn solution_one(input: &str) -> usize {
    let blueprint = parse(input);
    blueprint
        .iter()
        .map(|blueprint| max_geodes(blueprint))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * usize::from(geodes))
        .sum()
}

pub fn run() -> Result<(), GlobalError> {
    Ok(())
}

#[test]
fn challenge_one_day_19() -> Result<(), GlobalError> {
    let input = "assets/day19/sample.txt";
    let output = solution_one(input);
    assert_eq!(33, output);
    Ok(())
}
