use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);

        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let ((sr, sc), (dr, dc)) = get_start_end(&input);
        println!("grid:\n{:?}\nsr:{:?}", input, ((sr, sc), (dr, dc)));

        let mut inputc = input.clone();
        inputc[sr][sc] = 0;
        Self::bfs(&inputc, sr, sc, -28)
    }

    //copy from 2021 day 9
    fn bfs<T: std::ops::Sub + std::cmp::PartialEq + Copy + std::fmt::Display + std::convert::Into<i32>>(
        grid: &Vec<Vec<T>>,
        x: usize,
        y: usize,
        target: T,
    ) -> i32
    where
        <T as std::ops::Sub>::Output: PartialOrd<i32>,
    {
        let mut to_visit = vec![(x, y, 0)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let m = grid.len();
        let n = grid[0].len();
        let mut shortest_lengths: Vec<Vec<i32>> =
            (0..m).map(|_| std::iter::repeat(i32::MAX).take(n).collect()).collect();

        while to_visit.len() > 0 {
            let (visitx, visity, pathlen) = to_visit.remove(0);
            println!("visiting grid({},{})={} len {}", visitx, visity, grid[visitx][visity], pathlen);
            let nbrs = Self::nbrs(visitx, visity, grid);
            for (nx, ny) in nbrs {
                // println!("nbr {} {} = {} ", nx, ny, grid[nx][ny]);
                if grid[nx][ny] - grid[visitx][visity] <= 1 && !visited.contains(&(nx, ny)) {
                    to_visit.push((nx, ny, pathlen + 1));
                }
            }
            shortest_lengths[visitx][visity] = shortest_lengths[visitx][visity].min(pathlen);
            // let pl = shortest_lengths.get(&(visitx, visity));
            // if let Some(m) = pl {
            // shortest_lengths.insert((visitx, visity), *m.min(&pathlen));
            // } else {
            // shortest_lengths.insert((visitx, visity), pathlen);
            // }
            // shortest_lengths.entry((visitx, visity)).or_default(pathlen).
            if grid[visitx][visity] == target {
                println!("grid:\n");
                let gc: Vec<Vec<i32>> =
                    grid.iter().map(|r| r.iter().map(|x| -> i32 { *x.into() }).collect_vec()).collect_vec();
                Self::print_grid(&gc);
                println!("shortest lengths:\n");
                Self::print_grid(&shortest_lengths);
                return pathlen;
            }

            visited.insert((visitx, visity));
        }
        -1
    }

    fn print_grid(grid: &Vec<Vec<i32>>) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let mut v = grid[i][j];
                if v == i32::MAX {
                    v = -1;
                }
                print!("{:>4}", v);
            }
            println!();
        }
        println!();
    }

    //copy from 2021 day 9
    fn nbrs<T>(x: usize, y: usize, hts: &Vec<Vec<T>>) -> Vec<(usize, usize)> {
        let max_x = hts.len();
        let max_y = hts[0].len();
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if x != 0 {
            nbrs.push((x - 1, y));
        }
        if x + 1 < max_x {
            nbrs.push((x + 1, y));
        }
        if y != 0 {
            nbrs.push((x, y - 1));
        }
        if y + 1 < max_y {
            nbrs.push((x, y + 1));
        }

        nbrs
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}

fn get_start_end(input: &Vec<Vec<i32>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == ('S' as i32) - ('a' as i32) {
                start = (i, j)
            } else if input[i][j] == ('E' as i32) - ('a' as i32) {
                end = (i, j)
            }
        }
    }
    (start, end)
}
