use itertools::Itertools;
pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let grid = input.lines().map(|r| r.chars().map(|c| char::to_digit(c, 10).unwrap()).collect_vec()).collect_vec();
    grid
}
