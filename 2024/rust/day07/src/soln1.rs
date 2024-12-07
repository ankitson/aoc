use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use util::num::concat_num;

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

fn can_make(lhs: &Vec<usize>, idx: usize, target: usize, curr: usize, is_part2: bool) -> bool {
    if idx >= lhs.len() {
        return curr == target;
    }
    if curr > target {
        return false;
    }
    can_make(&lhs, idx + 1, target, curr + lhs[idx], is_part2)
        || can_make(&lhs, idx + 1, target, curr * lhs[idx], is_part2)
        || (is_part2 && can_make(&lhs, idx + 1, target, concat_num(curr, lhs[idx]), is_part2))
}

fn can_make_par(lhs: &Vec<usize>, target: usize, idx: usize, curr: usize, is_part2: bool, depth: usize) -> bool {
    if idx >= lhs.len() {
        return curr == target;
    }
    if curr >= target {
        return false;
    }

    let mut calls =
        vec![(lhs, target, idx + 1, curr + lhs[idx], is_part2), (lhs, target, idx + 1, curr * lhs[idx], is_part2)];
    if is_part2 {
        calls.push((lhs, target, idx + 1, concat_num(curr, lhs[idx]), is_part2))
    }
    let results: Vec<bool> = calls
        .into_par_iter()
        .map(|(lhs, target, idx, curr, is_part2)| {
            if depth < 4 {
                can_make_par(lhs, target, idx, curr, is_part2, depth + 1)
            } else {
                can_make(lhs, target, idx, curr, is_part2)
            }
        })
        .collect();

    results.into_iter().any(|result| result)
}

pub fn part1(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.iter().filter_map(|(lhs, rhs)| can_make(&lhs, 0, *rhs, 0, false).then(|| *rhs)).sum()
}

pub fn part1_par(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.par_iter().filter_map(|(lhs, rhs)| can_make_par(&lhs, *rhs, 1, lhs[0], false, 0).then(|| *rhs)).sum()
}

pub fn part2(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.iter().filter_map(|(lhs, rhs)| can_make(&lhs, 0, *rhs, 0, true).then(|| *rhs)).sum()
}

pub fn part2_par(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    eqns.par_iter().filter_map(|(lhs, rhs)| can_make_par(&lhs, *rhs, 1, lhs[0], true, 0).then(|| *rhs)).sum()
}
