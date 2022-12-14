use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<i32>>;
pub type Output = i32;

pub fn parse(raw_input: &str) -> Input {
    let input = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|c| ((c.chars().nth(0).expect(".") as i32) - ('a' as i32)))
                .collect_vec()
        })
        .collect_vec();

    input
}
