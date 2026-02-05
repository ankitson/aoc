use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;
use util::grid;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

fn find(grid: &Vec<Vec<char>>, ch: char) -> (usize, usize) {
    grid.iter().enumerate().find_map(|(row, line)| line.iter().position(|&c| c == ch).map(|col| (row, col))).unwrap()
}
pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let (sr, sc) = find(&grid, 'S');
    let (er, ec) = find(&grid, 'E');
    bfs(&grid, (sr, sc), (er, ec))
}

fn is_90_degree_turn(old_dir: (isize, isize), new_dir: (isize, isize)) -> bool {
    // Dot product should be 0 for 90 degree turn
    old_dir.0 * new_dir.0 + old_dir.1 * new_dir.1 == 0
}

fn bfs(grid: &Vec<Vec<char>>, (sr, sc): (usize, usize), (er, ec): (usize, usize)) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((sr, sc, 0, 0, vec![(sr, sc)], vec![], (0, 1)));
    let mut visited = HashSet::new();
    let mut best_score: Option<usize> = None;
    while to_visit.len() > 0 {
        let (cr, cc, fwds, turns, path, turns_taken, (pdr, pdc)) = to_visit.pop_front().unwrap();
        visited.insert((cr, cc, pdr, pdc));
        if grid[cr][cc] == 'E' {
            let score = fwds + turns * 1000;
            if score <= 102388 {
                println!(
                    "Reached E with fwd ={fwds} turn = {turns}, score = {score} path = {path:?} turns={turns_taken:?}"
                );
            }
            best_score = Some(best_score.map_or(score, |b| b.min(score)));
            continue;
        }

        for (nr, nc) in grid::nbrs4(cr, cc, grid.len(), grid[0].len()) {
            let (dr, dc) = (nr as isize - cr as isize, nc as isize - cc as isize);
            if visited.contains(&(nr, nc, dr, dc)) {
                continue;
            }
            if grid[nr][nc] == '#' {
                continue;
            }

            let is_turn = is_90_degree_turn((pdr, pdc), (dr, dc));
            let mut next_turns_taken = turns_taken.clone();
            if is_turn {
                next_turns_taken.push(((cr, cc), (nr, nc)));
            }
            let nturns = if is_turn { turns + 1 } else { turns };
            let nfwds = if is_turn { fwds + 1 } else { fwds + 1 };
            let mut npath = path.clone();
            npath.push((nr, nc));
            to_visit.push_back((nr, nc, nfwds, nturns, npath, next_turns_taken, (dr, dc)));
        }
    }
    best_score.unwrap()
    //102388
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
