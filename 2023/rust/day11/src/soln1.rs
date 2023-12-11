use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

pub type Input = (Vec<Vec<char>>, Vec<(usize, usize)>, Vec<usize>, Vec<usize>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid1 = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut galaxies = vec![];
    let mut extra_rows = vec![];
    let mut extra_cols = vec![];
    for i in 0..grid1.len() {
        let mut row_empty = true;
        for j in 0..grid1[i].len() {
            if grid1[i][j] == '#' {
                row_empty = false;
                galaxies.push((i, j));
            }
        }
        if row_empty {
            extra_rows.push(i);
        }
    }
    for j in 0..grid1[0].len() {
        let mut col_empty = true;
        for i in 0..grid1.len() {
            if grid1[i][j] == '#' {
                col_empty = false;
                break;
            }
        }
        if col_empty {
            extra_cols.push(j);
        }
    }
    (grid1, galaxies, extra_rows, extra_cols)
}

pub fn part1(raw_input: &str) -> Output {
    solve(raw_input, false)
}

pub fn part2(raw_input: &str) -> Output {
    solve(raw_input, true)
}

fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn grid_to_adjacency_matrix(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![0; grid.len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '#' {
                adj[i][j] = 1;
            }
        }
    }
    adj
}

pub fn solve(raw_input: &str, part2: bool) -> Output {
    let (_, galaxies, empty_rows, empty_cols) = parse(raw_input);
    let mut sum = 0;
    //Becase all galaxies are directly connected to each other, with a consistent metric
    //manhattan distance that follows triangle inequality, we dont need floyd-warshall
    //d(a,b) = d(a,k) + d(k,b)
    for (g1, g2) in (0..galaxies.len()).tuple_combinations() {
        let (r1, c1) = galaxies[g1];
        let (r2, c2) = galaxies[g2];
        let dx = (r1 as isize - r2 as isize).abs() as usize;
        let dy = (c1 as isize - c2 as isize).abs() as usize;
        let d = dx + dy;

        let smx = r1.min(r2);
        let bgx = r1.max(r2);
        let smy = c1.min(c2);
        let bgy = c1.max(c2);
        let extra_row = (smx..bgx).filter(|i| empty_rows.contains(i)).count();
        let extra_col = (smy..bgy).filter(|i| empty_cols.contains(i)).count();
        let mut extra = extra_row + extra_col;
        if part2 {
            extra *= 1000000 - 1;
        }

        sum += d + extra;
    }
    sum
}
