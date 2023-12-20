use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

pub type Input = Vec<Vec<usize>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid =
        input.lines().map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect_vec()).collect_vec();
    grid
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let out = dj2(&input, false);
    out.unwrap()
}

fn dj2(grid: &Vec<Vec<usize>>, part2: bool) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Visit {
        dist: Reverse<usize>,
        node: (usize, usize),
        num_consec: usize,
        last_dir: (isize, isize),
    }

    //Node (1,4) on sample is wrong distance is 22 but should be <=17
    //So min distance to (1,3) is 11, obtained by going right three times.
    //So going right again is not possible.
    //But we can get to (1,3) from above at dist of 13, which allows 17 to (1,4)
    let start = Visit { dist: Reverse(0), node: (0, 0), num_consec: 0, last_dir: (0, 0) };
    to_visit.push(start);
    while let Some(visit) = to_visit.pop() {
        //TODO: And why is this needed?
        if distances.contains_key(&(visit.node, visit.last_dir, visit.num_consec)) {
            continue;
        }
        distances
            .entry((visit.node, visit.last_dir, visit.num_consec))
            .and_modify(|e| *e = visit.dist.0.min(*e))
            .or_insert(visit.dist.0);

        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let mut can_go = true;
            let same_dir = (dr, dc) == visit.last_dir;
            let next_count = if same_dir { visit.num_consec + 1 } else { 1 };
            let rev_dir = (-dr, -dc) == visit.last_dir;
            if !part2 {
                can_go = !rev_dir && next_count <= 3;
            } else {
                let can_turn = visit.num_consec >= 4;
                let too_long = next_count > 10;
                let start = visit.last_dir == (0, 0);
                can_go = !rev_dir && !too_long && (same_dir || can_turn || start);
            }
            let nr = visit.node.0 as isize + dr;
            let nc = visit.node.1 as isize + dc;
            if nr.min(nc) >= 0 && nr < grid.len() as isize && nc < grid[0].len() as isize && can_go {
                // TODO: THIS IS IT, the line that fucks it up
                // let new_dist = visit.dist.0 + grid[nr as usize][nc as usize];
                // distances
                // .entry(((nr as usize, nc as usize), (dr, dc), visit.num_consec))
                // .and_modify(|e| *e = new_dist.min(*e))
                // .or_insert(new_dist);
                if distances.get(&((nr as usize, nc as usize), (dr, dc), next_count)).is_none() {
                    let next_visit = Visit {
                        dist: Reverse(visit.dist.0 + grid[nr as usize][nc as usize]),
                        node: (nr as usize, nc as usize),
                        num_consec: next_count,
                        last_dir: (dr, dc),
                    };
                    to_visit.push(next_visit);
                }
            }
        }
    }
    let end = (grid.len() - 1, grid[0].len() - 1);
    let mut best: Option<usize> = None;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut dist = 1000;
            for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                for num_consec in 0..11 {
                    dist = dist.min(*distances.get(&((i, j), dir, num_consec)).unwrap_or(&usize::MAX));
                    if (i, j) == end {
                        best = best.map(|b| b.min(dist)).or(Some(dist));
                    }
                }
            }
            // print!("{dist}  ");
        }
        // println!();
    }
    best
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
    let out = dj2(&input, true);
    out.unwrap()
}
