use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use util::grid::inbounds2z;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

fn points_are_collinear((r1, c1): (isize, isize), (r2, c2): (isize, isize)) -> Option<(i32, i32)> {
    return Some((1, 1));
}

fn draw_grid(locs: &FxHashMap<char, FxHashSet<(isize, isize)>>, nr: usize, nc: usize) {
    let mut grid = vec![vec!['.'; nc]; nr];
    for (_, positions) in locs {
        for (r, c) in positions {
            if *r >= 0 && *c >= 0 {
                grid[*r as usize][*c as usize] = '#';
            }
        }
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
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
    println!("grid = {num_rows} X {num_cols}");
    let mut antinode_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (antenna_type, antenna_locs) in antenna_locs.iter() {
        for loc_pair in antenna_locs.iter().combinations(2) {
            let (l1, l2) = (loc_pair[0], loc_pair[1]);
            if let Some((m, c)) = points_are_collinear(*l1, *l2) {
                let (dx, dy) = (l2.0 - l1.0, l2.1 - l1.1);
                let (p1x, p1y) = (l2.0 - 2 * dx, l2.1 - 2 * dy);
                let (p2x, p2y) = (l1.0 + 2 * dx, l1.1 + 2 * dy);
                println!("pair {:?} {:?} {:?} {:?} {:?}", antenna_type, l1, l2, (p1x, p1y), (p2x, p2y));
                if inbounds2z((p1x, p1y), num_rows, num_cols) {
                    println!("{p1x},{p1y} is inbounds");
                    antinode_locs.entry(*antenna_type).or_default().insert((p1x, p1y));
                }
                if inbounds2z((p2x, p2y), num_rows, num_cols) {
                    println!("{p2x},{p2y} is inbounds");

                    antinode_locs.entry(*antenna_type).or_default().insert((p2x, p2y));
                }
            }
        }
    }

    println!("{:?}", antinode_locs);
    draw_grid(&antinode_locs, num_rows as usize, num_cols as usize);
    let mut combined = FxHashSet::default();
    for vs in antinode_locs.values() {
        let valvec: Vec<(isize, isize)> = vs.iter().cloned().collect();
        combined.extend(valvec);
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
    println!("grid = {num_rows} X {num_cols}");
    let mut antinode_locs: FxHashMap<char, FxHashSet<(isize, isize)>> = FxHashMap::default();
    for (antenna_type, antenna_locs) in antenna_locs.iter() {
        for loc_pair in antenna_locs.iter().combinations(2) {
            let (l1, l2) = (loc_pair[0], loc_pair[1]);
            if let Some((m, c)) = points_are_collinear(*l1, *l2) {
                let (dx, dy) = (l2.0 - l1.0, l2.1 - l1.1);

                let (mut p1x, mut p1y) = (l2.0 - 2 * dx, l2.1 - 2 * dy);
                let (mut p2x, mut p2y) = (l1.0 + 2 * dx, l1.1 + 2 * dy);
                while inbounds2z((p1x, p1y), num_rows, num_cols) || inbounds2z((p2x, p2y), num_rows, num_cols) {
                    if inbounds2z((p1x, p1y), num_rows, num_cols) {
                        antinode_locs.entry(*antenna_type).or_default().insert((p1x, p1y));
                    }
                    if inbounds2z((p2x, p2y), num_rows, num_cols) {
                        antinode_locs.entry(*antenna_type).or_default().insert((p2x, p2y));
                    }
                    (p1x, p1y) = (p1x - dx, p1y - dy);
                    (p2x, p2y) = (p2x + dx, p2y + dy);
                }
            }
        }
    }

    println!("{:?}", antinode_locs);
    draw_grid(&antinode_locs, num_rows as usize, num_cols as usize);
    let mut combined = FxHashSet::default();
    for vs in antinode_locs.values() {
        let valvec: Vec<(isize, isize)> = vs.iter().cloned().collect();
        combined.extend(valvec);
    }
    for vs in antenna_locs.values() {
        let valvec: Vec<(isize, isize)> = vs.iter().cloned().collect();
        combined.extend(valvec);
    }
    combined.len()
}
