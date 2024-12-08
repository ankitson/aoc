use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use util::grid::{inbounds2z, print_grid_spcl_locs};

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut antenna_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char != '.' {
                antenna_locs.entry(*char).or_default().insert((ri.try_into().unwrap(), ci.try_into().unwrap()));
            }
        }
    }
    let num_rows: isize = grid.len().try_into().unwrap();
    let num_cols: isize = grid[0].len().try_into().unwrap();
    let mut antinode_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (antenna_type, antenna_locs) in antenna_locs.iter() {
        for loc_pair in antenna_locs.iter().combinations(2) {
            let (l1, l2) = (loc_pair[0], loc_pair[1]);
            let (dx, dy) = (l2.0 - l1.0, l2.1 - l1.1);
            let (p1x, p1y) = (l1.0 - dx, l1.1 - dy);
            let (p2x, p2y) = (l2.0 + dx, l2.1 + dy);
            if inbounds2z((p1x, p1y), num_rows, num_cols) {
                antinode_locs.entry(*antenna_type).or_default().insert((p1x, p1y));
            }
            if inbounds2z((p2x, p2y), num_rows, num_cols) {
                antinode_locs.entry(*antenna_type).or_default().insert((p2x, p2y));
            }
        }
    }

    let mut combined: FxHashSet<(usize, usize)> = FxHashSet::default();
    for locs in antinode_locs.values() {
        combined.extend(locs.iter().map(|(a, b)| (*a as usize, *b as usize)));
    }
    combined.len()
}

pub fn part2(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut antenna_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char != '.' {
                antenna_locs.entry(*char).or_default().insert((ri.try_into().unwrap(), ci.try_into().unwrap()));
            }
        }
    }
    let num_rows: isize = grid.len().try_into().unwrap();
    let num_cols: isize = grid[0].len().try_into().unwrap();
    let mut antinode_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (antenna_type, antenna_locs) in antenna_locs.iter() {
        for loc_pair in antenna_locs.iter().combinations(2) {
            let (l1, l2) = (loc_pair[0], loc_pair[1]);
            let (dx, dy) = (l2.0 - l1.0, l2.1 - l1.1);

            let (mut p1x, mut p1y) = (l1.0 - dx, l1.1 - dy);
            let (mut p2x, mut p2y) = (l2.0 + dx, l2.1 + dy);
            loop {
                let inbounds_p1 = inbounds2z((p1x, p1y), num_rows, num_cols);
                let inbounds_p2 = inbounds2z((p2x, p2y), num_rows, num_cols);
                if !inbounds_p1 && !inbounds_p2 {
                    break;
                }
                if inbounds_p1 {
                    antinode_locs.entry(*antenna_type).or_default().insert((p1x, p1y));
                }
                if inbounds_p2 {
                    antinode_locs.entry(*antenna_type).or_default().insert((p2x, p2y));
                }
                (p1x, p1y) = (p1x - dx, p1y - dy);
                (p2x, p2y) = (p2x + dx, p2y + dy);
            }
        }
    }

    let mut combined: FxHashSet<(usize, usize)> = FxHashSet::default();
    for locs in antinode_locs.values().chain(antenna_locs.values()) {
        combined.extend(locs.iter().map(|(a, b)| (*a as usize, *b as usize)));
    }
    combined.len()
}
