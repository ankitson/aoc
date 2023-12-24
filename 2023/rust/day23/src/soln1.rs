use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;
use util::grid;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    grid
}

fn all_parents(grid: &Input, r: usize, c: usize) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let mut parents: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut to_visit = VecDeque::from(vec![(r, c)]);
    let mut first = true;
    while to_visit.len() > 0 {
        let (r, c) = to_visit.pop_front().unwrap();
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= grid.len() as isize || nc < 0 || nc >= grid[0].len() as isize {
                continue;
            }
            let nbr = grid[nr as usize][nc as usize];
            println!("considered nbr = {} at {dr} {dc} from {r} {c}", nbr);
            if nbr != '#'
                && ((dr, dc) == (0, 1) && nbr == '>'
                    || (dr, dc) == (0, -1) && nbr == '<'
                    || (dr, dc) == (1, 0) && nbr == 'v'
                    || (dr, dc) == (-1, 0) && nbr == '^'
                    || first
                    || nbr == '.')
            {
                first = false;
                to_visit.push_back((nr as usize, nc as usize));
                parents
                    .entry((r as usize, c as usize))
                    .and_modify(|v| {
                        println!("insert new ito set");
                        v.insert((nr as usize, nc as usize));
                    })
                    .or_insert(HashSet::from([(nr as usize, nc as usize)]));
                println!("parents = {parents:?}");
                println!("inserting")
            }
        }
    }
    parents
}

fn find_all_paths(
    parents: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    (sr, sc): (usize, usize),
    (dr, dc): (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    if (sr, sc) == (dr, dc) {
        return vec![vec![(sr, sc)]];
    }
    let mut opaths = vec![];
    for (pdr, pdc) in parents.get(&(dr, dc)).unwrap() {
        let mut paths = find_all_paths(&parents, (sr, sc), (*pdr, *pdc));
        for path in paths.iter_mut() {
            path.push((dr, dc));
            opaths.push(path.to_vec().to_vec());
        }
    }
    opaths
}

fn pdfs(
    grid: &Input,
    (r, c): (usize, usize),
    (dr, dc): (usize, usize),
    seen: HashSet<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    // println!("recurse at {r} {c}");
    if r == dr && c == dc {
        return vec![vec![(r, c)]];
    }
    let mut opaths = vec![];
    for (pdr, pdc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nr = r as isize + pdr;
        let nc = c as isize + pdc;
        if nr < 0
            || nr >= grid.len() as isize
            || nc < 0
            || nc >= grid[0].len() as isize
            || seen.contains(&(nr as usize, nc as usize))
        {
            continue;
        }
        // if (r == 5 && c == 3) {
        // println!("adding {nr} {nc} {nbr} {pdr} {pdc}");
        // }
        let curr = grid[r as usize][c as usize];
        let nbr = grid[nr as usize][nc as usize];

        let curr_is_arrow = curr != '.' && curr != '#';
        let nbr_is_arrow = nbr != '.' && nbr != '#';

        let both = !curr_is_arrow && !nbr_is_arrow;
        let iampointy = (pdr, pdc) == (0, 1) && curr == '>'
            || (pdr, pdc) == (0, -1) && curr == '<'
            || (pdr, pdc) == (1, 0) && curr == 'v'
            || (pdr, pdc) == (-1, 0) && curr == '^';
        let onec = (curr_is_arrow && iampointy && !nbr_is_arrow);
        let other = (curr_is_arrow && iampointy && curr == nbr);

        if both || onec || other {
            if (r == 5 && c == 3) {
                println!("adding {nr} {nc} {nbr} {pdr} {pdc}");
            }
            let mut seen = seen.clone();
            seen.insert((nr as usize, nc as usize));
            let mut paths = pdfs(grid, (nr as usize, nc as usize), (dr, dc), seen);
            for path in paths.iter_mut() {
                path.insert(0, (r, c));
                opaths.push(path.to_vec().to_vec());
            }
        }
    }
    opaths
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let row0 = &grid[0];
    let sr = 0 as isize;
    let sc = grid[0].iter().position(|z| *z == '.').unwrap() as isize;
    let dc = grid[grid.len() - 1].iter().position(|z| *z == '.').unwrap() as isize;
    let dr = grid.len() as isize - 1;

    // let parents = all_parents(&grid, dr as usize, dc as usize);
    // println!("parents = {parents:?}");
    // let paths = find_all_paths(&parents, (sr as usize, sc as usize), (dr as usize, dc as usize));
    // println!("paths = {:?}", paths);

    let mut seen = HashSet::new();
    let paths = pdfs(&grid, (sr as usize, sc as usize), (dr as usize, dc as usize), seen);
    // println!("paths = {:?}", paths);
    println!("paths.len() = {:?}", paths.len());
    let mut maxsize = usize::MIN;
    for path in paths.iter() {
        if path.len() > maxsize {
            maxsize = path.len();
        }
    }
    let longest = paths.iter().filter(|p| p.len() == maxsize).collect_vec()[0];
    println!("Longest1 = {:?}", longest);
    maxsize

    // let mut to_visit = BinaryHeap::from(vec![(0, sr, sc, 0, 0)]);
    // // let mut to_visit = VecDeque::from(vec![(sr, sc, 0)]);
    // let mut seen = HashSet::new();
    // //run djikstras algorithm
    // while let Some((d, r, c, dr, dc)) = to_visit.pop() {
    //     if seen.contains(&(r, c, dr, dc)) {
    //         continue;
    //     }
    //     seen.insert((r, c, dr, dc));
    //     for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
    //         let nr = r + dr;
    //         let nc = c + dc;
    //         if nr < 0 || nr >= grid.len() as isize || nc < 0 || nc >= grid[0].len() as isize {
    //             continue;
    //         }
    //         if seen.contains(&(r, c, dr, dc)) {
    //             continue;
    //         }
    //         let nbr = grid[nr as usize][nc as usize];
    //         if nbr == '#' {
    //             continue;
    //         }
    //         if nbr != '.' {
    //             if (dr, dc) == (0, 1) && nbr == '>'
    //                 || (dr, dc) == (0, -1) && nbr == '<'
    //                 || (dr, dc) == (1, 0) && nbr == 'v'
    //                 || (dr, dc) == (-1, 0) && nbr == '^'
    //             {
    //                 to_visit.push((d + 1, nr, nc, dr, dc))
    //                 // return format!("{}", d);
    //             }
    //         }
    //     }
    // }
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
