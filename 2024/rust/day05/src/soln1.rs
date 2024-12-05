use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let mut rules: Vec<(usize, usize)> = vec![];
    let mut updates = vec![];
    let mut sec2 = false;
    for line in input.lines() {
        if line == "" {
            sec2 = true;
            continue;
        }
        if !sec2 {
            let pts = line.split("|").map(|x| x.parse().unwrap()).collect_vec();
            rules.push((pts[0], pts[1]));
        } else {
            let update: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect_vec();
            updates.push(update)
        }
    }
    (rules, updates)
}

pub fn part1(raw_input: &str) -> Output {
    let (rules, updates) = parse(raw_input);
    let mut result = 0;
    for mut update in updates {
        let midelem = update[update.len() / 2];
        let mut invalid = false;
        while update.len() > 0 {
            let item = update[0];
            for rule in &rules {
                if rule.1 == item && update.contains(&rule.0) {
                    invalid = true;
                }
            }
            update.remove(0);
        }
        if !invalid {
            result += midelem
        }
    }

    result
}

pub fn part2(raw_input: &str) -> Output {
    let (rules, updates) = parse(raw_input);
    let mut result = 0;
    let mut invalid_updates = vec![];
    for mut update in updates {
        let midelem = update[update.len() / 2];
        let mut invalid = false;
        let update_back = update.clone();
        while update.len() > 0 {
            let item = update[0];
            for rule in &rules {
                if rule.1 == item && update.contains(&rule.0) {
                    invalid = true;
                }
            }
            update.remove(0);
        }
        if invalid {
            invalid_updates.push(update_back)
        }
    }

    for mut invalid_update in invalid_updates {
        // println!("invalid update: {:?}", invalid_update);
        // let mut added_so_far = HashSet::new();
        let mut correct_update = vec![];
        // let mut candidate = true;
        let mut removed_idxs = HashSet::new();
        while correct_update.len() < invalid_update.len() {
            for (idx, item) in invalid_update.iter().enumerate() {
                let mut candidate = true;
                if removed_idxs.contains(&idx) {
                    continue;
                }
                for rule in &rules {
                    // let pred_idx = invalid_update.iter().position(|v| v == rule.0);
                    if rule.1 == *item && (invalid_update.contains(&rule.0) && !correct_update.contains(&rule.0)) {
                        candidate = false;
                        // println!("{:?} is not cand because rule {:?}", item, rule);
                    }
                }

                if candidate {
                    // println!("{:?} is cand ", item);
                    correct_update.push(*item);
                    removed_idxs.insert(idx);
                    // invalid_update.remove(idx);
                }
            }
        }
        println!("correct = {:?}", correct_update);
        result += correct_update[correct_update.len() / 2];
    }

    result
}
