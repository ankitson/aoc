use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|row| row.chars().collect_vec()).collect_vec();
    grid
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut starts = vec![];
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char == '0' {
                starts.push((ri, ci))
            }
        }
    }
    let mut total = 0;
    let mut end_locs = FxHashMap::default();
    for (sr, sc) in starts {
        let mut visited = FxHashSet::default();
        let mut acc = 0;
        bfs(&grid, sr, sc, &mut acc, &mut visited, &mut end_locs);
        println!("{acc} reachable from {sr},{sc}");
        total += acc;
    }
    total
}

fn bfs(
    grid: &Vec<Vec<char>>,
    sr: usize,
    sc: usize,
    acc: &mut usize,
    visited: &mut FxHashSet<(usize, usize)>,
    end_locs: &mut FxHashMap<(usize, usize), usize>,
) {
    println!("visit = ({sr},{sc}) acc={acc} visited={visited:?} end_locs={end_locs:?}");
    visited.insert((sr, sc));
    if grid[sr][sc] == '9' {
        *end_locs.entry((sr, sc)).or_default() += 1;
        *acc += 1;
        return;
    }
    let mut tot = 0;
    let nbrs = util::grid::nbrs4(sr, sc, grid.len(), grid[0].len());
    for nbr in nbrs {
        // if !visited.contains(&nbr) {
        if grid[nbr.0][nbr.1] == '.' {
            continue;
        }
        let nbrv = (grid[nbr.0][nbr.1]).to_digit(10).unwrap();
        let curv = (grid[sr][sc]).to_digit(10).unwrap();
        if nbrv > curv && nbrv - curv == 1 {
            bfs(&grid, nbr.0, nbr.1, acc, visited, end_locs);
        }
        // }
    }
    return;
}

pub fn part2(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut starts = vec![];
    for (ri, row) in grid.iter().enumerate() {
        for (ci, char) in row.iter().enumerate() {
            if *char == '0' {
                starts.push((ri, ci))
            }
        }
    }
    let mut total = 0;
    let mut end_locs = FxHashMap::default();
    for (sr, sc) in starts {
        let mut visited = FxHashSet::default();
        let mut acc = 0;
        bfs(&grid, sr, sc, &mut acc, &mut visited, &mut end_locs);
        println!("{acc} reachable from {sr},{sc}");
        println!("{end_locs:?}");
        total += acc;
    }
    total
}
