use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
    iter,
};

use crate::shared::parse;
use itertools::Itertools;

pub struct Soln1 {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    coord: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Soln1 {
    fn lookup_grid(grid: &Vec<Vec<u8>>, x: usize, y: usize, scale: usize) -> usize {
        if scale < 1 {
            panic!("illegal scale");
        }

        let size = grid.len();
        let vsize = size * scale;

        let grid_val = {
            if x < size && y < size {
                grid[x][y] as usize
            } else {
                let big_grid_x = x / size;
                let big_grid_y = y / size;
                let l1_dist = big_grid_x + big_grid_y;
                let cx = x % size;
                let cy = y % size;
                let gridu: usize = grid[cx][cy].into();
                let mut corresponding: usize = (gridu + l1_dist) % 10;
                corresponding += (gridu + l1_dist) / 10;
                corresponding
            }
        };

        grid_val
    }

    pub fn djikstra(grid: &Vec<Vec<u8>>, scale: usize, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
        let size = grid.len();
        let vsize = size * scale;
        let mut to_visit = BinaryHeap::new();
        to_visit.push(State { cost: 0, coord: (0, 0) });

        let mut costs = iter::repeat(iter::repeat(usize::MAX as usize).take(vsize).collect_vec())
            .take(vsize)
            .collect_vec();
        costs[0][0] = 0;

        while let Some(State { cost, coord }) = to_visit.pop() {
            if coord == to {
                return Some(cost);
            }

            if cost > costs[coord.0][coord.1] {
                continue;
            }

            for (nbrx, nbry) in Self::nbrs(coord.0, coord.1, vsize, vsize) {
                let nbr_state = State {
                    cost: cost + Self::lookup_grid(grid, nbrx, nbry, 5) as usize,
                    coord: (nbrx, nbry),
                };

                if nbr_state.cost < costs[nbrx][nbry] {
                    to_visit.push(nbr_state);
                    costs[nbrx][nbry] = nbr_state.cost;
                }
            }
        }

        None
    }

    pub fn part1(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        Self::djikstra(&grid, 1, (0, 0), (size - 1, size - 1)).unwrap()
    }

    pub fn part2(input: &str) -> usize {
        let grid = parse(input);
        let size = grid.len();
        Self::djikstra(&grid, 5, (0, 0), (size * 5 - 1, size * 5 - 1)).unwrap()
    }

    pub fn part1_slow(input: &str) -> usize {
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
                        let new_cost: usize = cost_to_here + (grid[nx][ny] as usize);
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
