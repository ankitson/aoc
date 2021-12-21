use itertools::Itertools;
use regex::Regex;

pub fn parse(input: &str) -> Vec<usize> {
    let mut lines = input.lines().filter(|l| !l.is_empty());
    let mut starts: Vec<usize> = Vec::new();
    let re = Regex::new(r"(\d+)($|\n)").unwrap();
    for cap in re.captures_iter(input) {
        let parsed = &cap[1].parse::<usize>().unwrap();
        starts.push(*parsed);
    }
    starts
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        assert_eq!(parse(include_str!("../inputs/sample.txt")), vec![4, 8]);
        assert_eq!(parse(include_str!("../inputs/day21.txt")), vec![8, 1]);
    }
}
