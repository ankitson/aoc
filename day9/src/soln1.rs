use crate::shared;
use std::collections::HashSet;

pub struct Soln1 {}
impl Soln1 {
    fn bfs(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut to_visit = vec![(x, y)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        while to_visit.len() > 0 {
            let (visitx, visity) = to_visit.remove(0);
            let nbrs = Self::nbrs(visitx, visity, grid);
            for (nx, ny) in nbrs {
                if grid[nx][ny] > grid[visitx][visity]
                    && grid[nx][ny] != 9
                    && !visited.contains(&(nx, ny))
                {
                    to_visit.push((nx, ny));
                }
            }
            visited.insert((visitx, visity));
        }
        visited
    }

    fn mut_bfs(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
        let mut to_visit = vec![(x, y)];
        let mut size: usize = 0;
        while to_visit.len() > 0 {
            let (visitx, visity) = to_visit.remove(0);
            let nbrs = Self::nbrs(visitx, visity, grid);
            for (nx, ny) in nbrs {
                if grid[nx][ny] > grid[visitx][visity] && grid[nx][ny] != 9 && grid[nx][ny] != 10 {
                    to_visit.push((nx, ny));
                }
            }
            if grid[visitx][visity] != 10 {
                size += 1;
            }
            grid[visitx][visity] = 10;
        }
        size
    }

    pub fn part1(input: &str) -> u32 {
        let heights = shared::parse(input);

        let mut risk: u32 = 0;
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                let nbr = Self::nbrs(i, j, &heights);
                let pt = heights[i][j];
                let mut lower = true;
                for (nx, ny) in nbr {
                    if pt >= heights[nx][ny] {
                        lower = false;
                    }
                }
                if lower {
                    risk += u32::from(pt) + 1;
                }
            }
        }
        risk
    }

    pub fn part2(input: &str) -> Vec<usize> {
        let mut heights = shared::parse(input);

        let mut islands: Vec<usize> = Vec::new();
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                let nbr = Self::nbrs(i, j, &heights);
                let pt = heights[i][j];
                let mut lower = true;
                for (nx, ny) in nbr {
                    if pt >= heights[nx][ny] {
                        lower = false;
                    }
                }
                if lower {
                    let island = Self::bfs(&mut heights, i, j);
                    islands.push(island.len());
                }
            }
        }
        islands.sort_unstable();
        let mut islands: Vec<usize> = islands.iter().rev().cloned().collect();
        assert!(islands.len() >= 3);
        islands.resize(3, 0);
        islands
    }

    pub fn part2_mut(input: &str) -> Vec<usize> {
        let mut heights = shared::parse(input);

        let mut islands: Vec<usize> = Vec::new();
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                let nbr = Self::nbrs(i, j, &heights);
                let pt = heights[i][j];
                let mut lower = true;
                for (nx, ny) in nbr {
                    if pt >= heights[nx][ny] {
                        lower = false;
                    }
                }
                if lower {
                    let island = Self::mut_bfs(&mut heights, i, j);
                    islands.push(island);
                }
            }
        }
        islands.sort_unstable();
        let mut islands: Vec<usize> = islands.iter().rev().cloned().collect();
        assert!(islands.len() >= 3);
        islands.resize(3, 0);
        islands
    }

    fn nbrs(x: usize, y: usize, hts: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
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
}
