use itertools::Itertools;

pub const EMPTY: u8 = 0u8;
pub const RIGHT: u8 = 1u8;
pub const DOWN: u8 = 2u8;
pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let lines = input.lines().filter(|l| l.len() > 1);
    let mut rows = vec![];
    for line in lines {
        let mut row = vec![];
        for char in line.chars() {
            match char {
                '.' => row.push(EMPTY),
                '>' => row.push(RIGHT),
                'v' => row.push(DOWN),
                _ => (),
            }
        }
        rows.push(row.clone());
        row.clear();
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::shared::{DOWN, EMPTY, RIGHT};

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        assert_eq!(parsed.len(), 9);
        assert!(parsed.iter().all(|r| r.len() == 10));
        assert_eq!(parsed[0][0], DOWN);
        assert_eq!(parsed[0][1], EMPTY);
        assert_eq!(parsed[0][9], RIGHT);
    }
}
