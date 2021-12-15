use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use crate::shared::parse;
use itertools::Itertools;

pub struct Soln1 {}

impl Soln1 {
    ///Shortest path in weighted graph
    /// Maintain list of costs cost[(x,y)] = weight of shortest path from 0,0 to (x,y)
    ///
    pub fn part1(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
        costs.insert((0, 0), 0);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
        // let mut to_visit_next: HashSet<(usize, usize)> = HashSet::new();
        while to_visit.len() > 0 {
            let (vx, vy) = to_visit.pop_back().unwrap();
            visited.insert((vx, vy));
            for (nx, ny) in Self::nbrs(vx, vy, size, size) {
                if visited.contains(&(nx, ny)) {
                    continue;
                }
                match costs.get_mut(&(nx, ny)).cloned() {
                    Some(existing_cost) => {
                        let cost_to_here = costs.get(&(vx, vy)).cloned().unwrap();
                        let mut new_cost: usize = cost_to_here + (grid[nx][ny] as usize);
                        let mut best_cost = existing_cost;
                        if new_cost < existing_cost {
                            best_cost = new_cost;
                        }
                        costs.insert((nx, ny), best_cost);
                    }
                    None => {
                        let cost_to_here = costs.get(&(vx, vy)).cloned().unwrap();
                        let mut new_cost: usize = cost_to_here + (grid[nx][ny] as usize);
                        costs.insert((nx, ny), new_cost);
                    }
                }
                if !to_visit.contains(&(nx, ny)) {
                    to_visit.push_front((nx, ny));
                }
                // if to_visit.is_empty() {
                //     to_visit = to_visit_next;
                //     to_visit_next = HashSet::new();
                // }
            }
        }

        Self::print_grid(&grid);
        // println!("{:?}", grid);
        println!("{:?}", costs);
        *costs.get(&(size - 1, size - 1)).unwrap()
    }

    fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                print!("{}", grid[i][j]);
            }
            println!();
        }
        println!();
    }

    fn nbrs(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
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

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}
