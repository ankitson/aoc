use std::fs;
use std::path::Path;

fn parse() -> Vec<i32> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join("day1_part1.txt");
    let contents = fs::read_to_string(path).expect("Could not read file");
    let lines: Vec<i32> = contents
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    lines
}

pub fn part1() -> i32 {
    let nums = parse();

    let mut i = 1;
    let mut prev: i32 = nums[0];
    let mut incr = 0;
    while i < nums.len() {
        if nums[i] > prev {
            incr += 1;
        }
        prev = nums[i];
        i += 1;
    }
    incr
}
