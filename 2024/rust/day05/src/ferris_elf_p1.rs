use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
pub fn run(input: &str) -> i64 {
    let mut rules: FxHashMap<i64, FxHashSet<i64>> = FxHashMap::default();
    let mut updates = vec![];
    let mut parsing_rules = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let pts = line.split("|").map(|x| x.parse::<i64>().unwrap()).collect_vec();
            rules.entry(pts[0]).or_default().insert(pts[1]);
        } else {
            let update = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect_vec();
            updates.push(update)
        }
    }
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
