use super::GlobalError;

pub fn run() -> Result<(), GlobalError> {
    let path = "assets/day20/input.txt";
    //let output = solution_one(path);
    let output = solution_two(path);
    println!("The first output is {output}");
    Ok(())
}

fn parse(path: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(path).unwrap();
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solution_one(path: &str) -> i64 {
    let nums = parse(path);
    let mut mixed: Vec<_> = (0..nums.len()).collect();
    for (idx, &num) in nums.iter().enumerate() {
        let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
        mixed.remove(mixed_idx);

        let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
        mixed.insert(new_mixed_idx, idx);
    }

    let zero_idx = nums.iter().position(|&num| num == 0).unwrap();
    let zero_mixed_idx = mixed
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
            let nums_idx = mixed[mixed_idx];
            nums[nums_idx]
        })
        .sum()
}

fn solution_two(path: &str) -> i64 {
    let decryption_key = 811_589_153;
    let nums: Vec<_> = parse(path).iter().map(|num| num * decryption_key).collect();
    let mut mixed: Vec<_> = (0..nums.len()).collect();
    for _ in 0..10 {
        for (idx, &num) in nums.iter().enumerate() {
            let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
            mixed.remove(mixed_idx);

            let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_mixed_idx, idx);
        }
    }
    let zero_idx = nums.iter().position(|&num| num == 0).unwrap();
    let zero_mixed_idx = mixed
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
            let nums_idx = mixed[mixed_idx];
            nums[nums_idx]
        })
        .sum()
}

#[test]
fn challenge_one_day_twenty() -> Result<(), GlobalError> {
    let path = "assets/day20/sample.txt";
    let output = solution_one(path);
    assert_eq!(output, 3);
    Ok(())
}
#[test]
fn challenge_two_day_twenty() -> Result<(), GlobalError> {
    let path = "assets/day20/sample.txt";
    let output = solution_two(path);
    assert_eq!(1623178306, output);
    Ok(())
}
