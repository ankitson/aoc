use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<Vec<char>>, (usize, usize));
pub type Output = usize;

pub fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = (i, j)
            }
        }
    }
    (grid, start)
}

pub fn part1(raw_input: &str, nsteps: usize) -> Output {
    let (grid, start) = parse(raw_input);
    let mut to_visit = VecDeque::from([start]);
    let mut next = VecDeque::new();
    let mut seen_per = HashSet::new();
    let mut steps = 0;
    while to_visit.len() > 0 && steps <= nsteps {
        let (vr, vc) = to_visit.pop_back().unwrap();
        seen_per.insert((vr, vc));
        for (dr, dc) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let nr = (vr as isize) + dr;
            let nc = (vc as isize) + dc;
            if nr.min(nc) >= 0
                && nr < grid.len() as isize
                && nc < grid[0].len() as isize
                && grid[nr as usize][nc as usize] != '#'
            {
                let nr = nr as usize;
                let nc = nc as usize;
                if !seen_per.contains(&(nr, nc)) && !next.contains(&(nr, nc)) {
                    next.push_back((nr, nc));
                }
            }
        }
        if to_visit.len() == 0 && steps < nsteps {
            to_visit = next;
            next = VecDeque::new();
            steps += 1;
            seen_per = HashSet::new();
        }
    }
    seen_per.len()
}

pub fn part2(raw_input: &str, nsteps: usize, print_intermediate: bool) -> Output {
    let (grid, start) = parse(raw_input);
    let (sr, sc) = (start.0 as isize, start.1 as isize);
    let mut to_visit = VecDeque::from([(sr, sc)]);
    let mut next: VecDeque<(isize, isize)> = VecDeque::new();
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    let mut seen_per: HashSet<(isize, isize)> = HashSet::new();
    let mut steps = 0;

    while to_visit.len() > 0 && steps <= nsteps {
        let (vr, vc) = to_visit.pop_back().unwrap();
        seen.insert((vr as isize, vc as isize));
        seen_per.insert((vr as isize, vc as isize));
        for (dr, dc) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let nr = (vr as isize) + dr;
            let nc = (vc as isize) + dc;
            let nrm = nr.rem_euclid(grid.len() as isize) as usize;
            let ncm = nc.rem_euclid(grid[0].len() as isize) as usize;
            if grid[nrm][ncm] == '.' || grid[nrm][ncm] == 'S' {
                if !seen_per.contains(&(nr, nc)) && !next.contains(&(nr, nc)) {
                    next.push_back((nr, nc));
                }
            }
        }
        if to_visit.len() == 0 && steps < nsteps {
            to_visit = next;
            next = VecDeque::new();
            let nseen = seen_per.len();
            if print_intermediate {
                println!("({steps},{nseen}),");
            }
            steps += 1;
            seen_per = HashSet::new();
        }
    }
    let nseen = seen_per.len();
    if print_intermediate {
        println!("({nsteps},{nseen})");
    }
    seen_per.len()
}
