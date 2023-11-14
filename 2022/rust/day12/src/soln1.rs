use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    fn get_neighbors(grid: &Vec<Vec<usize>>, (px, py): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let max_climb = grid[px][py] + 1;
        if px > 0 && max_climb >= grid[px - 1][py] {
            neighbors.push((px - 1, py));
        }
        if px < grid.len() - 1 && max_climb >= grid[px + 1][py] {
            neighbors.push((px + 1, py));
        }
        if py > 0 && max_climb >= grid[px][py - 1] {
            neighbors.push((px, py - 1));
        }
        if py < grid[0].len() - 1 && max_climb >= grid[px][py + 1] {
            neighbors.push((px, py + 1));
        }
        neighbors
    }
    fn dfs(grid: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from([start]);
        let mut next_layer: VecDeque<(usize, usize)> = VecDeque::new();
        let mut dist: usize = 0;
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        while !to_visit.is_empty() {
            let visit = to_visit.pop_front().unwrap();
            seen.insert(visit);
            if visit == end {
                return Some(dist);
            }
            let nbrs = Self::get_neighbors(&grid, visit);
            for nbr in nbrs {
                if !seen.contains(&nbr) && !next_layer.contains(&nbr) {
                    next_layer.push_back(nbr);
                }
            }
            if to_visit.is_empty() {
                dist += 1;
                to_visit = next_layer;
                next_layer = VecDeque::new();
            }
        }
        None
    }
    pub fn part1_core(input: &Input) -> Output {
        let (grid, start, end) = input;
        let res = Self::dfs(grid, *start, *end);
        res.unwrap()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let (grid, _, end) = input;
        let mut best_dist = usize::MAX;
        let start_positions = grid
            .iter()
            .enumerate()
            .flat_map(|(ri, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(ci, item)| if *item == 0 { Some((ri, ci)) } else { None })
                    .collect_vec()
            })
            .collect_vec();
        for (start_row, start_col) in start_positions {
            let dist = Self::dfs(grid, (start_row, start_col), *end).unwrap_or(usize::MAX);
            best_dist = best_dist.min(dist);
        }
        best_dist
    }
}
