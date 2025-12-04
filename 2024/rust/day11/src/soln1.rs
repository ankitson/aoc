use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

pub type Input = Vec<u64>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec()
}

fn num_digs(n: u64) -> u64 {
    n.to_string().len().try_into().unwrap()
}

fn split(n: u64) -> Vec<u64> {
    let nstr = n.to_string();
    let (a, b) = (nstr[0..nstr.len() / 2].parse().unwrap(), nstr[nstr.len() / 2..nstr.len()].parse().unwrap());
    (vec![a, b])
}

pub fn step(nums: &Vec<u64>) -> Vec<u64> {
    let mut next = vec![];
    for num in nums {
        let next_dig = match num {
            0 => vec![1],
            d if num_digs(*d) % 2 == 0 => split(*d),
            d => vec![d * 2024],
        };
        next.extend(next_dig)
    }
    next
}

pub fn step_mut(nums: &Vec<u64>) -> Vec<u64> {
    let mut next = vec![];
    for num in nums {
        let next_dig = match num {
            0 => vec![1],
            d if num_digs(*d) % 2 == 0 => split(*d),
            d => vec![d * 2024],
        };
        next.extend(next_dig)
    }
    next
}

pub fn part1(raw_input: &str) -> Output {
    let mut nums = parse(raw_input);
    for i in 0..25 {
        nums = step(&nums);
    }
    nums.len()
}

pub fn part1_fast(raw_input: &str) -> Output {
    let nums = parse(raw_input);
    let mut result = nums.len() as u64;
    for num in &nums {
        step_num(*num, 25, &mut result)
    }
    result as usize
}

pub fn part1_rec(raw_input: &str) -> Output {
    let nums = parse(raw_input);
    let mut result = 0;
    let mut cache = FxHashMap::default();
    for num in &nums {
        result += step_rec(*num, 25, 1, &mut cache);
    }
    result as usize
}

pub fn step2(nums: &Vec<u64>) -> Vec<u64> {
    let mut next = vec![];
    for num in nums {
        let next_dig = match num {
            0 => vec![1],
            d if num_digs(*d) % 2 == 0 => split(*d),
            d => vec![d * 2024],
        };
        next.extend(next_dig)
    }
    next
}

pub fn part2(raw_input: &str) -> Output {
    let nums = parse(raw_input);
    let mut result = 0;
    let mut cache = FxHashMap::default();
    for num in &nums {
        result += step_rec(*num, 75, 1, &mut cache);
    }
    result as usize
}

pub fn step_rec(num: u64, iters: u64, acc: u64, cache: &mut FxHashMap<(u64, u64), u64>) -> u64 {
    if iters == 0 {
        return acc;
    }
    if let Some(val) = cache.get(&(num, iters)) {
        return *val;
    }

    let next = match num {
        0 => step_rec(1, iters - 1, acc, cache),
        d if num_digs(d) % 2 == 0 => {
            let spls = split(d);
            let (a, b) = (spls[0], spls[1]);
            step_rec(a, iters - 1, acc, cache) + step_rec(b, iters - 1, acc, cache)
        }
        d => step_rec(d * 2024, iters - 1, acc, cache),
    };
    cache.insert((num, iters), next);
    next
}

pub fn step_num(num: u64, iters: u64, acc: &mut u64) -> () {
    if iters == 0 {
        return;
    }

    match num {
        0 => step_num(1, iters - 1, acc),
        d if num_digs(d) % 2 == 0 => {
            let spls = split(d);
            let (a, b) = (spls[0], spls[1]);
            *acc += 1;
            step_num(a, iters - 1, acc);
            step_num(b, iters - 1, acc);
        }
        d => step_num(d * 2024, iters - 1, acc),
    }
}
