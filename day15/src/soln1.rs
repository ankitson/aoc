use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    iter,
};

use crate::shared::parse;
use itertools::{iproduct, Itertools};

pub struct Soln1 {}

impl Soln1 {
    pub fn modulo(a: usize, b: usize) -> usize {
        let ai = i32::try_from(a).unwrap();
        let bi = i32::try_from(b).unwrap();
        let res = ((ai % bi) + bi) % bi;
        res as usize
    }

    pub fn part2(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        let vsize = size * 5;
        let mut costs = iter::repeat(iter::repeat(usize::MAX as usize).take(vsize).collect_vec())
            .take(vsize)
            .collect_vec();
        costs[0][0] = 0;

        let mut coords = iproduct!(0..vsize, 0..vsize).collect_vec();
        coords.sort_unstable_by(|(x1, y1), (x2, y2)| (*x1 + *y1).cmp(&(*x2 + *y2)));
        for (x, y) in coords {
            for (nx, ny) in Self::nbrs(x, y, vsize, vsize) {
                let grid_val: usize = {
                    if nx < size && ny < size {
                        grid[nx][ny] as usize
                    } else {
                        // 0-9: 0
                        // 10-19: 1
                        // 20-29: 2
                        let big_grid_x = nx / 10;
                        let big_grid_y = ny / 10;
                        let l1_dist = big_grid_x + big_grid_y;
                        let cx = Self::modulo(nx, 10);
                        let cy = Self::modulo(ny, 10);
                        let gridu: usize = grid[cx][cy].into();
                        let mut corresponding: usize = Self::modulo(gridu + l1_dist, 10);
                        if gridu + l1_dist >= 10 {
                            corresponding += 1;
                        }
                        // println!(
                        //     "Mapping {} {} to {} {} with offset {}. grid ={}, vgrid = {}",
                        //     nx, ny, cx, cy, l1_dist, grid[cx][cy], corresponding
                        // );
                        corresponding
                    }
                };
                costs[nx][ny] = costs[nx][ny].min(costs[x][y] + grid_val)
            }
        }
        // Self::print_grid(&costs);
        costs[vsize - 1][vsize - 1]
    }

    pub fn part1_fast(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        let mut costs = iter::repeat(iter::repeat(usize::MAX as usize).take(size).collect_vec())
            .take(size)
            .collect_vec();
        costs[0][0] = 0;

        let mut coords = iproduct!(0..size, 0..size).collect_vec();
        coords.sort_unstable_by(|(x1, y1), (x2, y2)| (*x1 + *y1).cmp(&(*x2 + *y2)));
        for (x, y) in coords {
            for (nx, ny) in Self::nbrs(x, y, size, size) {
                costs[nx][ny] = costs[nx][ny].min(costs[x][y] + (grid[nx][ny] as usize))
            }
        }
        // Self::print_grid(&costs);
        costs[size - 1][size - 1]
    }

    pub fn part1(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
        costs.insert((0, 0), 0);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from(vec![(0, 0)]);
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
            }
        }

        // Self::print_grid(&grid);
        // println!("{:?}", grid);
        // println!("{:?}", costs);
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
}
