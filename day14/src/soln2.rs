use crate::shared::{parse, Rule};
use itertools::Itertools;
use std::collections::HashMap;

type PairCount = HashMap<(char, char), usize>;
type RuleMap = HashMap<(char, char), char>;

pub fn build_map(poly: &str) -> PairCount {
    let mut pair_count: PairCount = HashMap::new();
    for (c1, c2) in poly.chars().tuple_windows() {
        let count = pair_count.entry((c1, c2)).or_insert(0);
        *count += 1;
    }
    pair_count
}

pub fn build_rules(rules: &[Rule]) -> RuleMap {
    let mut rule_map: RuleMap = HashMap::new();
    for rule in rules {
        let mut rule_chars = rule.from.chars();
        let c1 = rule_chars.next().unwrap();
        let c2 = rule_chars.next().unwrap();
        let to = rule.insert.chars().next().unwrap();
        rule_map.insert((c1, c2), to);
    }
    rule_map
}

pub fn iterate(pair_count: &mut PairCount, rule_map: &RuleMap) -> PairCount {
    let mut new_pair: PairCount = HashMap::new();
    for ((c1, c2), n) in pair_count {
        if rule_map.contains_key(&(*c1, *c2)) {
            let c3 = rule_map.get(&(*c1, *c2)).unwrap();
            *new_pair.entry((*c1, *c3)).or_insert(0) += *n;
            *new_pair.entry((*c3, *c2)).or_insert(0) += *n;
        } else {
            *new_pair.entry((*c1, *c2)).or_insert(0) += *n;
        }
    }
    new_pair
}

pub fn char_counts(first_char: char, map: &PairCount) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for char_idx in 0..26 {
        let char = (b'A' + char_idx) as char;
        counts.insert(char, 0);
    }
    *counts.get_mut(&first_char).unwrap() += 1;
    for ((c1, c2), v) in map {
        *counts.get_mut(c2).unwrap() += v;
    }
    counts
}

pub fn run(input: &str, n: usize) -> usize {
    let (poly, rules) = parse(input);
    let mut pair_count = build_map(poly);
    let rule_map = build_rules(&rules);

    for _ in 0..n {
        pair_count = iterate(&mut pair_count, &rule_map);
    }
    println!("final map: {:?}", pair_count);
    let char_counts = char_counts(poly.chars().nth(0).unwrap(), &pair_count);
    let mx = char_counts
        .values()
        .filter(|v| **v > 0)
        .minmax()
        .into_option()
        .unwrap();
    println!("min max: {:?}", mx);
    *mx.1 - *mx.0
}

pub fn part1(input: &str) -> usize {
    run(input, 10)
}
pub fn part2(input: &str) -> usize {
    run(input, 40)
}
