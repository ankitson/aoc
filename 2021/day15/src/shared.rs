use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| (char.to_digit(10).unwrap() as u8))
                .collect_vec()
        })
        .collect_vec()
}
