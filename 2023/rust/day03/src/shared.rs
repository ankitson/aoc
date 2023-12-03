use itertools::Itertools;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
    grid
}
