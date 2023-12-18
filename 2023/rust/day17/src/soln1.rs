use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use regex::Regex;
use util::grid as gr;

pub type Input = Vec<Vec<usize>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid =
        input.lines().map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect_vec()).collect_vec();
    grid
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    // dfs(&input, vec![], (0, 0), 0, &HashSet::new());
    // todo!()
    let out = djikstra(&input);
    println!("Output from djikstra = {out:?}");
    out.unwrap()
}

fn djikstra(grid: &Vec<Vec<usize>>) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0usize), (0usize, 0usize), 0, (0, 0))); //dist, coord, num consec, last_dir

    let mut reached_from = HashMap::new();
    reached_from.insert((0, 0), (-1isize, -1isize));
    while !heap.is_empty() {
        let (dist, (vr, vc), num_consec, last_dir) = heap.pop().unwrap();
        if !seen.insert(((vr, vc), last_dir, num_consec)) {
            continue;
        }
        if !reached_from.contains_key(&(vr, vc)) {
            reached_from.insert((vr, vc), ((vr as isize) - last_dir.0, (vc as isize) - last_dir.1));
        }
        println!("Visit {vr}, {vc} at dist {dist:?} nc {num_consec} last {last_dir:?}");
        if vr == 1 && vc == 6 {
            println!("REACHED MAP  = {reached_from:?}");
            let mut path: Vec<(isize, isize)> = vec![];
            let mut node = (vr as isize, vc as isize);
            path.push((vr as isize, vc as isize));
            while node != (0, 0) {
                let (vr, vc) = node;
                let mut parent = reached_from.get(&(vr as usize, vc as usize)).unwrap();
                path.push(*parent);
                node = *parent;
                // println!("node = {node:?}");
            }
            path.reverse();
            println!("PATH TO DEST = {path:?}");
            //grid.len() - 1 && vc == grid[0].len() - 1 {
            return Some(dist.0);
        }
        let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dir in dirs {
            if num_consec == 3 && dir == last_dir {
                continue;
            }
            let next_consec = if dir == last_dir { num_consec + 1 } else { 1 };
            let (nr, nc) = (vr as isize + dir.0, vc as isize + dir.1);
            if nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid[0].len() as isize {
                let ndist = dist.0 + grid[nr as usize][nc as usize];
                println!("pushing {nr} {nc} at dist {ndist}");
                heap.push((
                    Reverse(dist.0 + grid[nr as usize][nc as usize]),
                    (nr as usize, nc as usize),
                    next_consec,
                    dir,
                ));
            }
        }
    }
    return None;
}
fn dfs(
    grid: &Vec<Vec<usize>>,
    last3: Vec<(isize, isize)>,
    curr: (usize, usize),
    heat: usize,
    seen: &HashSet<(usize, usize)>,
) -> usize {
    let (r, c) = curr;

    // println!("visit {r} {c} seen = {seen:?}");
    let mut seen = seen.clone();
    if last3.len() > 0 {
        let dir = last3.get(2).unwrap_or(&(-1, -1));
        // println!("visit from dir {dir:?}");
        seen.insert((r, c)); //, last3[last3.len() - 1].0, last3[last3.len() - 1].1));
    }
    // println!("seen after = {seen:?}");
    if r == grid.len() - 1 && c == grid[0].len() - 1 {
        return heat + grid[r][c];
    }
    let mut dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let same_dir = last3.iter().all_equal();
    if same_dir {
        dirs = dirs.into_iter().filter(|d| *d != *last3.get(0).unwrap_or(&(10isize, 10isize))).collect_vec();
    }
    let mut best_heat = usize::MAX;
    for (dr, dc) in dirs {
        let cr = (r as isize) + dr;
        let cc = (c as isize) + dc;
        if cr >= 0 && cr < grid.len() as isize && cc >= 0 && cc < grid[0].len() as isize {
            if !seen.contains(&(cr as usize, cc as usize)) {
                // if !seen.contains(&(cr as usize, cc as usize, dr, dc)) {
                let mut last3 = last3.clone();
                if last3.len() == 3 {
                    last3.remove(0);
                }
                last3.push((dr, dc));
                let this_heat = dfs(grid, last3, (cr as usize, cc as usize), heat + grid[r][c], &seen);
                best_heat = best_heat.max(this_heat);
            }
        }
    }
    best_heat
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
