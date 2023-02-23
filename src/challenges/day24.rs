use std::collections::{BinaryHeap, HashMap, HashSet};

use super::GlobalError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Blizzard(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    pos: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        use Direction::*;
        let mut neighbours = Vec::new();
        if self.row > 0 {
            neighbours.push(self.add_dir(&Up));
        }
        if self.col < cols - 1 {
            neighbours.push(self.add_dir(&Right));
        }
        if self.row < rows - 1 {
            neighbours.push(self.add_dir(&Down));
        }
        if self.col > 0 {
            neighbours.push(self.add_dir(&Left));
        }
        neighbours
    }

    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

fn parse(input: &str) -> (HashMap<Coord, Tile>, usize, usize) {
    let mut map = HashMap::new();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = Coord { row, col };
            let tile = match c {
                '#' => Tile::Wall,
                '^' => Tile::Blizzard(Direction::Up),
                'v' => Tile::Blizzard(Direction::Down),
                '<' => Tile::Blizzard(Direction::Left),
                '>' => Tile::Blizzard(Direction::Right),
                _ => panic!("invalid input"),
            };
            map.insert(coord, tile);
        }
    }
    (map, rows, cols)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn blitz_maps(
    map: &HashMap<Coord, Tile>,
    rows: usize,
    cols: usize,
    max_time: usize,
) -> HashMap<usize, HashSet<Coord>> {
    let mut cache = HashMap::new();

    let mut blizzards: Vec<(Coord, Direction)> = map
        .iter()
        .filter_map(|(pos, tile)| match tile {
            Tile::Wall => None,
            Tile::Blizzard(dir) => Some((*pos, *dir)),
        })
        .collect();

    let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
    cache.insert(0, coords);

    for time in 1..max_time {
        for (coord, dir) in blizzards.iter_mut() {
            *coord = coord.add_dir(dir);
            match dir {
                Direction::Left => {
                    if coord.col == 0 {
                        coord.col = cols - 2;
                    }
                }

                Direction::Right => {
                    if coord.col == cols - 1 {
                        coord.col = 1;
                    }
                }
                Direction::Up => {
                    if coord.row == 0 {
                        coord.row = rows - 2;
                    }
                }
                Direction::Down => {
                    if coord.row == rows - 1 {
                        coord.row = 1;
                    }
                }
            }
        }
        let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
        cache.insert(time, coords);
    }
    cache
}

fn part_one(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();

    let (map, rows, cols) = parse(&input);
    let walls: HashSet<Coord> = map
        .iter()
        .filter(|(_, tile)| **tile == Tile::Wall)
        .map(|(pos, _)| *pos)
        .collect();
    let lcm = lcm(rows - 2, cols - 2);
    let blizzard_maps = blitz_maps(&map, rows, cols, lcm);
    let start = Coord { row: 0, col: 1 };
    let end = Coord {
        row: rows - 1,
        col: cols - 2,
    };

    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: 0,
        pos: start,
    });
    seen.insert((start, 0));

    while let Some(Node { cost, pos }) = pq.pop() {
        if pos == end {
            return cost;
        }

        let new_cost = cost + 1;
        let blizzards = &blizzard_maps[&(new_cost % lcm)];

        let candidates = pos
            .neighbours(rows, cols)
            .into_iter()
            .chain(std::iter::once(pos))
            .filter(|coord| !walls.contains(coord))
            .filter(|coord| !blizzards.contains(coord));

        for new_pos in candidates {
            if seen.insert((new_pos, new_cost)) {
                pq.push(Node {
                    cost: new_cost,
                    pos: new_pos,
                });
            }
        }
    }
    usize::MAX
}

struct MapInfo {
    rows: usize,
    cols: usize,
    walls: HashSet<Coord>,
    blizzard_maps: HashMap<usize, HashSet<Coord>>,
    repeats_at: usize,
}

fn shortest(from: Coord, to: Coord, start_time: usize, map_info: &MapInfo) -> usize {
    let MapInfo {
        rows,
        cols,
        walls,
        blizzard_maps,
        repeats_at,
    } = map_info;
    let mut pq = BinaryHeap::new();

    let mut seen = HashSet::new();

    pq.push(Node {
        cost: start_time,
        pos: from,
    });
    seen.insert((from, start_time));

    while let Some(Node { cost, pos }) = pq.pop() {
        if pos == to {
            return cost;
        }

        let new_cost = cost + 1;
        let blizzards = &blizzard_maps[&(new_cost % repeats_at)];

        let candidates = pos
            .neighbours(*rows, *cols)
            .into_iter()
            .chain(std::iter::once(pos))
            .filter(|coord| !walls.contains(coord))
            .filter(|coord| !blizzards.contains(coord));

        for new_pos in candidates {
            if seen.insert((new_pos, new_cost)) {
                pq.push(Node {
                    cost: new_cost,
                    pos: new_pos,
                })
            }
        }
    }
    usize::MAX
}

fn part_two(path: &str) -> usize {
    let input = std::fs::read_to_string(path).unwrap();

    let (map, rows, cols) = parse(&input);
    let walls: HashSet<Coord> = map
        .iter()
        .filter(|(_, tile)| **tile == Tile::Wall)
        .map(|(pos, _)| *pos)
        .collect();

    let lcm = lcm(rows - 2, cols - 2);
    let blizzard_maps = blitz_maps(&map, rows, cols, lcm);
    let start = Coord { row: 0, col: 1 };
    let end = Coord {
        row: rows - 1,
        col: cols - 2,
    };

    let map_info = MapInfo {
        rows,
        cols,
        repeats_at: lcm,
        walls,
        blizzard_maps,
    };
    let there = shortest(start, end, 0, &map_info);
    let back = shortest(end, start, there, &map_info);
    shortest(start, end, back, &map_info)
}

pub fn run() -> Result<(), GlobalError> {
    let path = "assets/day24/input.txt";
    //    let output = part_one(path);
    //    println!("{output} is the first result");
    let output = part_two(path);
    println!("{output} is the first result");
    Ok(())
}

#[test]
fn challenge_one_day_24() -> Result<(), GlobalError> {
    let path = "assets/day24/sample.txt";
    let output = part_one(path);
    assert_eq!(output, 18);
    Ok(())
}

#[test]
fn challenge_two_day_24() -> Result<(), GlobalError> {
    let path = "assets/day24/sample.txt";
    let output = part_two(path);
    assert_eq!(output, 54);
    Ok(())
}
