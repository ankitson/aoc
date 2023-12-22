use std::collections::{HashSet, VecDeque};

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

fn nbrs(x: usize, y: usize, hts: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let max_x = hts.len();
    let max_y = hts[0].len();
    let cands = vec![
        // (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() - 1),
        (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() + 0),
        // (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() + 1),
        (i64::try_from(x).unwrap() + 0, i64::try_from(y).unwrap() - 1),
        (i64::try_from(x).unwrap() + 0, i64::try_from(y).unwrap() + 1),
        // (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() - 1),
        (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() + 0),
        // (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() + 1),
    ];

    let mut nbrs2: Vec<(usize, usize)> = Vec::new();
    for (a, b) in cands {
        if a < 0 || b < 0 {
            continue;
        }
        let aa = a.try_into().unwrap();
        let bb = b.try_into().unwrap();
        if aa < max_x && bb < max_y {
            let ro: &Vec<char> = &hts[aa];
            let pt = ro[bb];
            if (pt == '.' || pt == 'S') {
                nbrs2.push((aa, bb));
            }
        }
    }
    nbrs2
}

pub fn part1(raw_input: &str) -> Output {
    let (grid, start) = parse(raw_input);
    let mut to_visit = VecDeque::from([start]);
    let mut next = VecDeque::new();
    let mut seen = HashSet::new();
    let mut seen_per = HashSet::new();
    let mut steps = 0;
    let NSTEPS = 64 + 1;
    while (to_visit.len() > 0 && steps < NSTEPS) {
        let (vr, vc) = to_visit.pop_back().unwrap();
        seen.insert((vr, vc));
        seen_per.insert((vr, vc));
        for (nr, nc) in nbrs(vr, vc, &grid) {
            if !seen_per.contains(&(nr, nc)) && !next.contains(&(nr, nc)) {
                next.push_back((nr, nc));
            }
        }
        if to_visit.len() == 0 {
            to_visit = next;
            next = VecDeque::new();
            steps += 1;
            let nseen = seen_per.len();
            println!("{nseen} seen after steps {steps}");
            // println!("next layer = {to_visit:?}");
            if steps == NSTEPS {
                break;
            }
            seen_per = HashSet::new();
        }
    }
    seen_per.len()
}

fn pos_mod(a: isize, b: isize) -> usize {
    (((a % b) + b) % b) as usize
}

pub fn part2(raw_input: &str) -> Output {
    let (grid, start) = parse(raw_input);
    let (sr, sc) = (start.0 as isize, start.1 as isize);
    let mut to_visit = VecDeque::from([(sr, sc)]);
    let mut next = VecDeque::new();
    let mut seen = HashSet::new();
    let mut seen_per = HashSet::new();
    let mut steps = 0;
    let NSTEPS = 26501365 + 1;

    let mut prev_values = vec![];
    while to_visit.len() > 0 && steps < NSTEPS {
        let (vr, vc) = to_visit.pop_back().unwrap();
        seen.insert((vr as isize, vc as isize));
        seen_per.insert((vr as isize, vc as isize));
        for (dr, dc) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let nrm = pos_mod((vr as isize) + dr, grid.len() as isize);
            let ncm = pos_mod((vc as isize) + dc, grid[0].len() as isize);
            let nr = (vr as isize) + dr;
            let nc = (vc as isize) + dc;
            if grid[nrm][ncm] == '.' || grid[nrm][ncm] == 'S' {
                if !seen_per.contains(&(nr, nc)) && !next.contains(&(nr, nc)) {
                    next.push_back((nr, nc));
                }
            }
        }
        // for (nr, nc) in nbrs(vr, vc, &grid) {

        // }
        if to_visit.len() == 0 {
            to_visit = next;
            next = VecDeque::new();
            steps += 1;
            let nseen = seen_per.len();
            println!("{nseen} seen after steps {steps}");

            //MINE IS N+1, AFTER STPES = 101 corresponds to after 100
            // println!("next layer = {to_visit:?}");
            if steps == NSTEPS {
                break;
            }

            let prev_position = prev_values.iter().find_position(|seen_set| **seen_set == seen_per);
            if let Some((earlier_idx, earlier_set)) = prev_position {
                println!("after {steps}, repeated at {earlier_idx}");

                panic!();
            }
            prev_values.push(seen_per.clone());

            seen_per = HashSet::new();
        }
    }
    seen_per.len()
}
