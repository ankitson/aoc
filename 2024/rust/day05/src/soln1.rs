use itertools::Itertools;
use rustc_hash::*;

pub type Input = (FxHashMap<usize, FxHashSet<usize>>, Vec<Vec<usize>>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let mut rules: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut updates = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let pts =
                line.split_once("|").map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())).unwrap();
            rules.entry(pts.0).or_default().insert(pts.1);
        } else {
            let update = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
            updates.push(update)
        }
    }
    (rules, updates)
}

pub fn part1(raw_input: &str) -> Output {
    let (rules, updates) = parse(raw_input);
    let mut result = 0;
    for update in updates {
        let mut invalid = false;
        'outer: for i in 0..update.len() {
            if rules.get(&update[i]).iter().any(|v| update[0..i].iter().any(|n| v.contains(&n))) {
                invalid = true;
                break 'outer;
            }
        }
        if !invalid {
            result += update[update.len() / 2]
        }
    }
    result
}

pub fn part2(raw_input: &str) -> Output {
    todo!();
    // let (rules, updates) = parse(raw_input);
    // let mut result = 0;
    // let mut invalid_updates = vec![];
    // for update in updates {
    //     let mut invalid = false;
    //     let update_back = update.clone();
    //     for i in 0..update.len() {
    //         for rule in &rules {
    //             if rule.1 == update[i] && update[i..].contains(&rule.0) {
    //                 invalid = true;
    //                 break;
    //             }
    //         }
    //         if invalid {
    //             invalid_updates.push(update_back.clone());
    //             break;
    //         }
    //     }
    // }

    // for mut invalid_update in invalid_updates {
    //     let mut correct_update = vec![];
    //     while correct_update.len() < invalid_update.len() {
    //         for (idx, item) in invalid_update.iter().enumerate() {
    //             let mut candidate = true;
    //             if correct_update.contains(item) {
    //                 continue;
    //             }
    //             for rule in &rules {
    //                 if rule.1 == *item && (invalid_update.contains(&rule.0) && !correct_update.contains(&rule.0)) {
    //                     candidate = false;
    //                 }
    //             }

    //             if candidate {
    //                 correct_update.push(*item);
    //             }
    //         }
    //     }
    //     result += correct_update[correct_update.len() / 2];
    // }

    // result
}
