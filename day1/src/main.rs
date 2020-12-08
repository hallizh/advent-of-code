use std::fs;

fn main() {
    let filename = "/home/halli/dev/advent-of-code-2020/day1/src/data.xmasdev";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let v: Vec<u32> = contents
        .trim_end_matches("\n")
        .split('\n')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let p1 = part1(v.clone());
    let p2 = part2(v);

    println!(
        "Results from part one: \t{}\nResults from part two: \t{}",
        p1, p2
    )
}

// Problem: Find the two numbers in a list that sum up to 2020
//          Take these two numbers and multiply them together.
//          Sample data in data.xmasdev
fn part1(nums: Vec<u32>) -> u32 {
    for x in 0..nums.len() {
        for y in x..nums.len() {
            if y == x {
                continue;
            }
            if nums[x] + nums[y] == 2020 {
                return nums[x] * nums[y];
            }
        }
    }
    0
}

// Problem: Find the three numbers in a list that sum up to 2020
//          Take these three numbers and multiply them together.
//          Sample data in data.xmasdev
fn part2(nums: Vec<u32>) -> u32 {
    for x in 0..nums.len() {
        for y in 0..nums.len() {
            for z in x..nums.len() {
                if y == x && y == z {
                    continue;
                }
                if nums[x] + nums[y] + nums[z] == 2020 {
                    return nums[x] * nums[y] * nums[z];
                }
            }
        }
    }
    0
}
