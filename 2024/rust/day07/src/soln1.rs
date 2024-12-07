use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<(Vec<usize>, usize)>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (rhs, rest) = line.split_once(": ").unwrap();
            let lhs = rest.split_whitespace().map(|x| x.parse().unwrap()).collect_vec();
            (lhs, rhs.parse().unwrap())
        })
        .collect_vec()
}

fn can_make(lhs: &Vec<usize>, target: usize, idx: usize, curr: usize, is_part2: bool) -> bool {
    if idx >= lhs.len() {
        return curr == target;
    }
    let mut can = can_make(lhs, target, idx + 1, curr + lhs[idx], is_part2)
        || can_make(lhs, target, idx + 1, curr * lhs[idx], is_part2);
    if is_part2 {
        can |= can_make(lhs, target, idx + 1, (curr.to_string() + &lhs[idx].to_string()).parse().unwrap(), is_part2)
    }
    can
}
pub fn part1(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.iter().filter_map(|(lhs, rhs)| can_make(&lhs, *rhs, 1, lhs[0], false).then(|| *rhs)).sum()
}

pub fn part2(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.iter().filter_map(|(lhs, rhs)| can_make(&lhs, *rhs, 1, lhs[0], true).then(|| *rhs)).sum()
}
