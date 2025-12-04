use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let nbrs = util::grid::nbrs8(i, j, grid.len(), grid[0].len());
            let roll_nbrs = nbrs.iter().filter(|(nr, nc)| grid[*nr][*nc] == '@').count();
            if roll_nbrs < 4 {
                count += 1
            }
        }
    }
    count
}

pub fn part2(raw_input: &str) -> Output {
    let mut grid = parse(raw_input);
    let mut count = 0;
    let mut prev_count = 1;
    while count != prev_count {
        let mut removed_this_round = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '@' {
                    continue;
                }
                let nbrs = util::grid::nbrs8(i, j, grid.len(), grid[0].len());
                let roll_nbrs = nbrs.iter().filter(|(nr, nc)| grid[*nr][*nc] == '@').count();
                if roll_nbrs < 4 {
                    removed_this_round += 1;
                    grid[i][j] = '.';
                }
            }
        }
        prev_count = count;
        count += removed_this_round;
    }
    count
}
