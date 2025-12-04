use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<isize>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.replace("L", "-").replace("R", "+"))
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec()
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut pos = 50isize;
    let mut num_zeros = 0;
    for offset in &input {
        pos = (pos + offset).rem_euclid(100);
        if pos == 0 {
            num_zeros += 1;
        }
    }
    num_zeros
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
