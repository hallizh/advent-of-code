use std::fs;
// Problem: Find the two numbers in a list that sum up to 2020
//          Take these two numbers and multiply them.
//          Sample data in data.xmas
fn main() {
    println!("Hello, world!");
    let filename = "/home/halli/dev/advent-of-code-2020/day1/src/input.xmas";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let v: Vec<u32> = contents
        .trim_end_matches("\n")
        .split('\n')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    'outer: for x in 0..v.len() {
        for y in x..v.len() {
            if y == x {
                continue;
            }
            if v[x] + v[y] == 2020 {
                let answer = v[x] * v[y];
                println!("Answer: {}", answer);

                break 'outer;
            }
        }
    }
}
