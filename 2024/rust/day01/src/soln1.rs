use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<u32>, Vec<u32>);
pub type Output = u32;

pub fn parse(input: &str) -> Input {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for line in input.lines() {
        let parts = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec();
        l1.push(parts[0]);
        l2.push(parts[1])
    }
    return (l1, l2);
}

pub fn part1(raw_input: &str) -> Output {
    let (mut l1, mut l2) = parse(raw_input);
    l1.sort();
    l2.sort();
    let mut d = 0;
    for i in 0..l1.len() {
        d += l1[i].abs_diff(l2[i]);
    }
    d
}

pub fn part2(raw_input: &str) -> Output {
    let (l1, l2) = parse(raw_input);
    let mut freq_map = std::collections::HashMap::new();
    for &num in &l2 {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    let mut similarity = 0;
    for &num in &l1 {
        similarity += &num * freq_map.get(&num).unwrap_or(&0);
    }
    similarity
}
