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
    let mut pos = 50;
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
    let mut num_zeros = 0usize;
    let mut pos = 50;
    for &offset in &input {
        if offset < 0 {
            num_zeros += (offset.abs() as usize) / 100;
            if pos != 0 && offset.abs() % 100 >= pos {
                num_zeros += 1;
            }
            pos = (pos + offset).rem_euclid(100);
        } else {
            pos += offset;
            num_zeros += (pos as usize) / 100;
            pos = pos.rem_euclid(100);
        }
    }
    num_zeros
}
