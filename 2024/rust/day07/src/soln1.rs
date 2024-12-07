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

fn can_make(lhs: &Vec<usize>, ops: Vec<usize>, target: usize, idx: usize, curr: usize) -> bool {
    if idx >= lhs.len() {
        return curr == target;
    }
    let mut ops_left = ops.clone();
    ops_left.push(0); //+
    let p1 = curr + lhs[idx];
    let mut ops_right = ops.clone();
    ops_right.push(1); //*
    let p2 = curr * lhs[idx];
    can_make(lhs, ops_left, target, idx + 1, p1) || can_make(lhs, ops_right, target, idx + 1, p2)
}
pub fn part1(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    let mut total = 0;
    for (lhs, rhs) in eqns {
        if can_make(&lhs, vec![], rhs, 1, lhs[0]) {
            total += rhs;
        }
    }
    total
}

fn can_make2(lhs: &Vec<usize>, ops: Vec<usize>, target: usize, idx: usize, curr: usize) -> bool {
    if idx >= lhs.len() {
        return curr == target;
    }
    let mut ops_left = ops.clone();
    ops_left.push(0); //+
    let p1 = curr + lhs[idx];
    let mut ops_right = ops.clone();
    ops_right.push(1); //*
    let p2 = curr * lhs[idx];
    let mut ops_third = ops.clone();
    ops_third.push(2);
    let p3: usize = (curr.to_string() + &lhs[idx].to_string()).parse().unwrap();
    can_make2(lhs, ops_left, target, idx + 1, p1)
        || can_make2(lhs, ops_right, target, idx + 1, p2)
        || can_make2(lhs, ops_third, target, idx + 1, p3)
}

pub fn part2(raw_input: &str) -> Output {
    let eqns = parse(raw_input);
    let mut total = 0;
    for (lhs, rhs) in eqns {
        if can_make2(&lhs, vec![], rhs, 1, lhs[0]) {
            total += rhs;
        }
    }
    total
}
