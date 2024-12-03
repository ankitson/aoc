use itertools::Itertools;
use regex::Regex;

pub type Input = String;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.to_string()
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for capture in re.captures_iter(&input) {
        let n1: usize = capture[1].parse().unwrap();
        let n2: usize = capture[2].parse().unwrap();
        result += n1 * n2
    }
    result
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't)|(do)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for capture in re.captures_iter(&input) {
        match capture.get(0).unwrap().as_str() {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                if enabled {
                    let n1: usize = capture[1].parse().unwrap();
                    let n2: usize = capture[2].parse().unwrap();
                    result += n1 * n2
                }
            }
        }
    }
    result
}
