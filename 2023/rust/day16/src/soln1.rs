use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

use util::grid;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &'static str) -> Input {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    grid
}

fn rot(dx: isize, dy: isize, cw: bool) -> (isize, isize) {
    match ((dx, dy), cw) {
        ((0, 1), true) => (1, 0),
        ((1, 0), true) => (0, -1),
        ((0, -1), true) => (-1, 0),
        ((-1, 0), true) => (0, 1),
        ((0, 1), false) => (-1, 0),
        ((1, 0), false) => (0, 1),
        ((0, -1), false) => (1, 0),
        ((-1, 0), false) => (0, -1),
        _ => panic!("illegal"),
    }
}

pub fn part1(raw_input: &'static str) -> Output {
    let grid = parse(raw_input);
    let mut seen = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0isize, 0isize, (0isize, 1isize)));
    while to_visit.len() > 0 {
        let (vx, vy, (dx, dy)) = to_visit.pop_front().unwrap();

        if vx < 0 || vx >= grid.len() as isize || vy < 0 || vy >= grid[0].len() as isize {
            continue;
        }
        seen.insert((vx, vy, dx, dy), true);
        match (grid[vx as usize][vy as usize], (dx, dy)) {
            ('.', (_, _)) => {
                if !seen.contains_key(&(vx + dx, vy + dy, dx, dy)) {
                    to_visit.push_back((vx + dx, vy + dy, (dx, dy)))
                }
            }
            ('/', (a, b)) => {
                let (ndx, ndy) = match (a, b) {
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    _ => panic!("illegal"),
                };
                if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                    to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                }
            }
            ('\\', (a, b)) => {
                let (ndx, ndy) = match (a, b) {
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    _ => panic!("illegal"),
                };
                if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                    to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                }
            }
            ('|', (-1, 0)) | ('|', (1, 0)) => to_visit.push_back((vx + dx, vy + dy, (dx, dy))),
            ('|', (0, 1)) | ('|', (0, -1)) => {
                let (ndx1, ndy1) = rot(dx, dy, true);
                let (ndx2, ndy2) = rot(dx, dy, false);
                if !seen.contains_key(&(vx + ndx1, vy + ndy1, ndx1, ndy1)) {
                    to_visit.push_back((vx + ndx1, vy + ndy1, (ndx1, ndy1)));
                }
                if !seen.contains_key(&(vx + ndx2, vy + ndy2, ndx2, ndy2)) {
                    to_visit.push_back((vx + ndx2, vy + ndy2, (ndx2, ndy2)));
                }
            }
            ('-', (0, 1)) | ('-', (0, -1)) => to_visit.push_back((vx + dx, vy + dy, (dx, dy))),
            ('-', (-1, 0)) | ('-', (1, 0)) => {
                let (ndx1, ndy1) = rot(dx, dy, true);
                let (ndx2, ndy2) = rot(dx, dy, false);
                if !seen.contains_key(&(vx + ndx1, vy + ndy1, ndx1, ndy1)) {
                    to_visit.push_back((vx + ndx1, vy + ndy1, (ndx1, ndy1)));
                }
                if !seen.contains_key(&(vx + ndx2, vy + ndy2, ndx2, ndy2)) {
                    to_visit.push_back((vx + ndx2, vy + ndy2, (ndx2, ndy2)));
                }
            }
            _ => panic!("illegal"),
        }
    }

    let mut total = 0;

    let mut seen2 = vec![vec![false; grid[0].len()]; grid.len()];
    for ((x, y, dx, dy), b) in seen {
        if b {
            seen2[x as usize][y as usize] = true;
        }
    }
    for row in seen2 {
        for item in row {
            if item {
                total += 1;
            }
        }
    }
    total
}

pub fn part2(raw_input: &'static str) -> Output {
    let grid = parse(raw_input);

    let mut best = 0;
    let mut start_pos = vec![];
    for i in 0..grid.len() {
        start_pos.push((i, 0));
        start_pos.push((i, grid[0].len() - 1))
    }
    for j in 0..grid[0].len() {
        start_pos.push((0, j));
        start_pos.push((grid.len() - 1, j));
    }

    for pos in start_pos {
        let (x, y) = pos;
        let mut dirs = vec![];
        if x == 0 {
            dirs.push((1, 0));
        } else if x == grid.len() - 1 {
            dirs.push((-1, 0));
        }

        if y == 0 {
            dirs.push((0, 1))
        } else if y == grid[0].len() - 1 {
            dirs.push((0, -1))
        }

        for (sdx, sdy) in dirs {
            let mut to_visit = VecDeque::new();
            let mut seen = HashMap::new();
            to_visit.push_back((x as isize, y as isize, (sdx, sdy)));
            while to_visit.len() > 0 {
                let (vx, vy, (dx, dy)) = to_visit.pop_front().unwrap();

                if (vx < 0 || vx >= grid.len() as isize || vy < 0 || vy >= grid[0].len() as isize) {
                    continue;
                }
                seen.insert((vx, vy, dx, dy), true);
                match (grid[vx as usize][vy as usize], (dx, dy)) {
                    ('.', (_, _)) => {
                        if !seen.contains_key(&(vx + dx, vy + dy, dx, dy)) {
                            to_visit.push_back((vx + dx, vy + dy, (dx, dy)))
                        }
                    }
                    ('/', (a, b)) => {
                        let (ndx, ndy) = match (a, b) {
                            (0, 1) => (-1, 0),
                            (0, -1) => (1, 0),
                            (1, 0) => (0, -1),
                            (-1, 0) => (0, 1),
                            _ => panic!("illegal"),
                        };
                        if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                            to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                        }
                    }
                    ('\\', (a, b)) => {
                        let (ndx, ndy) = match (a, b) {
                            (0, 1) => (1, 0),
                            (0, -1) => (-1, 0),
                            (1, 0) => (0, 1),
                            (-1, 0) => (0, -1),
                            _ => panic!("illegal"),
                        };
                        if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                            to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                        }
                    }
                    ('|', (-1, 0)) | ('|', (1, 0)) => to_visit.push_back((vx + dx, vy + dy, (dx, dy))),
                    ('|', (0, 1)) | ('|', (0, -1)) => {
                        let (ndx1, ndy1) = rot(dx, dy, true);
                        let (ndx2, ndy2) = rot(dx, dy, false);
                        if !seen.contains_key(&(vx + ndx1, vy + ndy1, ndx1, ndy1)) {
                            to_visit.push_back((vx + ndx1, vy + ndy1, (ndx1, ndy1)));
                        }
                        if !seen.contains_key(&(vx + ndx2, vy + ndy2, ndx2, ndy2)) {
                            to_visit.push_back((vx + ndx2, vy + ndy2, (ndx2, ndy2)));
                        }
                    }
                    ('-', (0, 1)) | ('-', (0, -1)) => to_visit.push_back((vx + dx, vy + dy, (dx, dy))),
                    ('-', (-1, 0)) | ('-', (1, 0)) => {
                        let (ndx1, ndy1) = rot(dx, dy, true);
                        let (ndx2, ndy2) = rot(dx, dy, false);
                        if !seen.contains_key(&(vx + ndx1, vy + ndy1, ndx1, ndy1)) {
                            to_visit.push_back((vx + ndx1, vy + ndy1, (ndx1, ndy1)));
                        }
                        if !seen.contains_key(&(vx + ndx2, vy + ndy2, ndx2, ndy2)) {
                            to_visit.push_back((vx + ndx2, vy + ndy2, (ndx2, ndy2)));
                        }
                    }
                    _ => panic!("illegal"),
                }
            }

            let mut total = 0;
            let mut seen2 = vec![vec![false; grid[0].len()]; grid.len()];
            for ((x, y, dx, dy), b) in seen {
                if b {
                    seen2[x as usize][y as usize] = true;
                }
            }
            for row in seen2 {
                for item in row {
                    if item {
                        total += 1;
                    }
                }
            }
            best = best.max(total);
        }
    }
    best
}
