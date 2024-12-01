use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<u64>, Vec<u64>);
pub type Output = String;

pub fn parse(input: &str) -> Input {
    let mut l1 = vec![];
    let mut l2 = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect_vec();
        let n1: u64 = parts[0].parse().expect("Failed to parse number");
        let n2: u64 = parts[1].parse().expect("Failed to parse number");
        l1.push(n1);
        l2.push(n2);
    }
    return (l1, l2);
}

pub fn part1(raw_input: &str) -> Output {
    let (mut l1, mut l2) = parse(raw_input);
    l1.sort();
    l2.sort();
    let mut d = 0;
    for i in 0..l1.len() {
        let dx = (l1[i] as i64 - l2[i] as i64).abs() as u64;
        d += dx
    }
    d.to_string()
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
    similarity.to_string()
}
