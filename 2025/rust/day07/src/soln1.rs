use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<Vec<char>>, (usize, usize));
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let (mut sr, mut sc) = (0, 0);
    'outer: for row_idx in 0..grid.len() {
        let row = &grid[row_idx];
        for col_idx in 0..row.len() {
            if row[col_idx] == 'S' {
                (sr, sc) = (row_idx, col_idx);
                break 'outer;
            }
        }
    }
    (grid, (sr, sc))
}

pub fn part1(raw_input: &str) -> Output {
    let (grid, (sr, sc)) = parse(raw_input);
    let (mut br, mut bc) = (sr, sc);
    sim_beam((sr, sc), &grid, &mut HashSet::new())
}

fn sim_beam((sr, sc): (usize, usize), grid: &Vec<Vec<char>>, mut visited: &mut HashSet<(usize, usize)>) -> usize {
    if sr >= grid.len() - 1 || sc < 0 || sc >= grid[0].len() {
        return 0;
    }
    if (visited.contains(&(sr, sc))) {
        return 0;
    }
    visited.insert((sr, sc));
    if grid[sr][sc] == '^' {
        println!("split at {sr:?},{sc:?}");
        return sim_beam((sr + 1, sc - 1), grid, &mut visited) + sim_beam((sr + 1, sc + 1), grid, &mut visited) + 1;
    } else if grid[sr][sc] == '.' || grid[sr][sc] == 'S' {
        return sim_beam((sr + 1, sc), grid, &mut visited);
    } else {
        panic!("illegal ")
    }
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
