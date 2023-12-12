use itertools::Itertools;
use regex::Regex;

pub type Coord = (usize, usize);
pub type Input = String;
pub type Output = String;

pub fn parse(input: &str) -> (Vec<Vec<char>>, Coord) {
    let lines: Vec<Vec<char>> = input.split('\n').filter(|x| x.len() > 1).map(|x| x.chars().collect()).collect();

    let mut start_idx: Option<(usize, usize)> = None;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == 'S' {
                start_idx = Some((i, j));
                break;
            }
        }
    }
    (lines, start_idx.unwrap())
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    println!("P1 Input = {input:?}");
    todo!()
}

pub fn part2(raw_input: &str) -> Output {
    let (grid, start) = parse(raw_input);

    println!("P1 Input = {input:?}");
    todo!()
}
