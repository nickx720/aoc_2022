use std::collections::HashSet;

use super::GlobalError;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        for dimension in [Dimension::X, Dimension::Y, Dimension::Z] {
            for offset in [-1, 1] {
                let mut neighbour = self.clone();
                match dimension {
                    Dimension::X => neighbour.x += offset,
                    Dimension::Y => neighbour.y += offset,
                    Dimension::Z => neighbour.z += offset,
                }
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

fn parse(path: &str) -> HashSet<Coord> {
    let input = std::fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|s| s.parse().unwrap());
            Coord {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            }
        })
        .collect()
}

enum Dimension {
    X,
    Y,
    Z,
}

fn bounds(cubes: &HashSet<Coord>) -> [Coord; 2] {
    cubes.iter().fold(
        [Coord::default(), Coord::default()],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        },
    )
}

fn exposed(cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let bounds = bounds(cubes);
    let mut exposed = HashSet::new();

    let start = Coord::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(coord) = stack.pop() {
        for neighbour in coord.neighbours() {
            if cubes.contains(&neighbour) || !neighbour.in_bounds(&bounds) {
                continue;
            }
            if seen.insert(neighbour) {
                stack.push(neighbour);
                exposed.insert(neighbour);
            }
        }
    }
    exposed
}

fn solution_one(path: &str) -> usize {
    let cubes = parse(path);
    cubes
        .iter()
        .flat_map(|coord| coord.neighbours())
        .filter(|coord| !cubes.contains(coord))
        .count()
}

fn solution_two(path: &str) -> usize {
    let cubes = parse(path);
    let exposed = exposed(&cubes);
    cubes
        .iter()
        .flat_map(|coord| coord.neighbours())
        .filter(|coord| exposed.contains(coord))
        .count()
}
pub fn run() -> Result<(), GlobalError> {
    let input = "assets/day18/input.txt";
    let output = solution_two(input);
    println!("The output is {output}");
    Ok(())
}

#[test]
fn challenge_one_day_18() -> Result<(), GlobalError> {
    let input = "assets/day18/sample.txt";
    let output = solution_one(input);
    assert_eq!(output, 64);
    Ok(())
}
#[test]
fn challenge_two_day_18() -> Result<(), GlobalError> {
    let input = "assets/day18/sample.txt";
    let output = solution_two(input);
    assert_eq!(output, 58);
    Ok(())
}
