use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|ln| ln.chars().collect_vec()).collect_vec();
    grid
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut count = 0;
    for i in 0..nrows {
        let row = &grid[i];
        for j in 0..ncols - 3 {
            let subs = row[j..j + 4].iter().collect::<String>();
            if subs == "XMAS" {
                count += 1;
            }
            if subs == "SAMX" {
                count += 1
            }
        }
    }
    for j in 0..ncols {
        for i in 0..nrows - 3 {
            if grid[i][j] == 'X' && grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S' {
                count += 1;
            }
            if grid[i][j] == 'S' && grid[i + 1][j] == 'A' && grid[i + 2][j] == 'M' && grid[i + 3][j] == 'X' {
                count += 1;
            }
        }
    }
    let mut d = 0;
    while d < nrows + ncols {
        let mut last_seen = VecDeque::new();
        for i in 0..nrows {
            for j in 0..ncols {
                if i + j == d {
                    last_seen.push_back(grid[i][j]);
                    if last_seen.len() == 4 {
                        let subs = last_seen.iter().collect::<String>();
                        if subs == "XMAS" || subs == "SAMX" {
                            count += 1
                        }
                        last_seen.pop_front();
                    }
                }
            }
        }

        d += 1;
    }
    let mut d = 0;
    while d < nrows + ncols {
        let mut last_seen = VecDeque::new();
        for i in 0..nrows {
            for j in 0..ncols {
                if i + (ncols - 1 - j) == d {
                    last_seen.push_back(grid[i][j]);
                    if last_seen.len() == 4 {
                        let subs = last_seen.iter().collect::<String>();
                        if subs == "XMAS" || subs == "SAMX" {
                            count += 1
                        }
                        last_seen.pop_front();
                    }
                }
            }
        }
        d += 1;
    }
    count
}

pub fn part2(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut count = 0;
    for i in 0..nrows - 2 {
        for j in 0..ncols - 2 {
            if grid[i][j] == 'M'
                && grid[i][j + 2] == 'S'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j] == 'M'
                && grid[i + 2][j + 2] == 'S'
            {
                count += 1;
            } else if grid[i][j] == 'S'
                && grid[i][j + 2] == 'S'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j] == 'M'
                && grid[i + 2][j + 2] == 'M'
            {
                count += 1;
            } else if grid[i][j] == 'S'
                && grid[i][j + 2] == 'M'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j] == 'S'
                && grid[i + 2][j + 2] == 'M'
            {
                count += 1;
            } else if grid[i][j] == 'M'
                && grid[i][j + 2] == 'M'
                && grid[i + 1][j + 1] == 'A'
                && grid[i + 2][j] == 'S'
                && grid[i + 2][j + 2] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}
