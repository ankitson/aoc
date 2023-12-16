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

// /   up goes right => cw 90deg
// / down goes left => cw 90

pub fn part1(raw_input: &'static str) -> Output {
    let grid = parse(raw_input);
    // let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut seen = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0isize, 0isize, (0isize, 1isize)));
    while to_visit.len() > 0 {
        let (vx, vy, (dx, dy)) = to_visit.pop_front().unwrap();

        if (vx < 0 || vx >= grid.len() as isize || vy < 0 || vy >= grid[0].len() as isize) {
            continue;
        }
        println!("VX = {vx} VY = {vy} dx = {dx} dy = {dy}");
        seen.insert((vx, vy, dx, dy), true);
        // seen[vx as usize][vy as usize] = true;
        // if (vx as isize + dx) >= grid.len() as isize
        //     || (vy as isize + dy) >= grid[0].len() as isize
        //     || (vx as isize + dx) < 0
        //     || (vy as isize + dy) < 0
        // {
        //     //todo
        //     continue;
        // }
        // let nbr = grid[((vx as isize) + dx) as usize][((vy as isize) + dy) as usize];
        // let vx = vx as isize;
        // let vy = vy as isize;
        match (grid[vx as usize][vy as usize], (dx, dy)) {
            ('.', (_, _)) => {
                if !seen.contains_key(&(vx + dx, vy + dy, dx, dy)) {
                    to_visit.push_back((vx + dx, vy + dy, (dx, dy)))
                }
            }
            // ('/', ()) //UP GOES CW, RIGhT GOES ANTICW, LEFT GOES ANTICW, DOWN GOES CW
            ('/', (a, b)) => {
                let (mut ndx, mut ndy) = (0, 0);
                (ndx, ndy) = match (a, b) {
                    (0, 1) => (-1, 0), //BEAM GOING RIGHT GOES UP
                    (0, -1) => (1, 0), //BEAM GOING LEFT GOES DOWN
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    _ => panic!("illegal"),
                };
                // if a != 0 {
                //     //GOING UP OR DOWN
                //     (ndx, ndy) = rot(dx, dy, true);
                // } else {
                //     (ndx, ndy) = rot(dx, dy, false);
                // }
                println!("dx after rot at right mirror = {ndx} {ndy}");

                if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                    to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                }
            }
            //UP OGES ANTICW, DOWN GOES ANTI CW
            ('\\', (a, b)) => {
                let (ndx, ndy) = match (a, b) {
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    _ => panic!("illegal"),
                };

                println!("SAW \\ at {vx} {vy} going {ndx} {ndy}");
                // if b != 0 {
                // (ndx, ndy) = rot(dx, dy, true);
                // } else {
                // (ndx, ndy) = rot(dx, dy, false);
                // }

                // let (ndx, ndy) = rot(dx, dy, false);
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

    // println!("FINISHED SEEN\n={seen:?}");
    let mut total = 0;

    println!("seen\n{seen:?}");
    let mut seen2 = vec![vec![false; grid[0].len()]; grid.len()];
    for ((x, y, dx, dy), b) in seen {
        if b {
            seen2[x as usize][y as usize] = true;
        }
    }
    grid::print_grid(&seen2);
    for row in seen2 {
        for item in row {
            if item {
                print!("#");
                total += 1;
            } else {
                print!(".")
            }
            // if item {
            // }
        }
        println!();
    }
    total
    // println!("Part 1 Input = {input:?}");
}

pub fn part2(raw_input: &'static str) -> Output {
    let grid = parse(raw_input);

    let mut best = 0;
    let mut start_pos = vec![];
    for i in 0..grid.len() {
        start_pos.push((i, 0));
        start_pos.push((i, grid[0].len() - 1))
        // for j in 0..grid[0].len() {
        //
        //    /     }
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

        println!("will try dirs = {dirs:?} for pos = {pos:?}");

        for (sdx, sdy) in dirs {
            let mut to_visit = VecDeque::new();
            let mut seen = HashMap::new();
            to_visit.push_back((x as isize, y as isize, (sdx, sdy)));
            while to_visit.len() > 0 {
                let (vx, vy, (dx, dy)) = to_visit.pop_front().unwrap();

                if (vx < 0 || vx >= grid.len() as isize || vy < 0 || vy >= grid[0].len() as isize) {
                    continue;
                }
                // println!("VX = {vx} VY = {vy} dx = {dx} dy = {dy}");
                seen.insert((vx, vy, dx, dy), true);
                // seen[vx as usize][vy as usize] = true;
                // if (vx as isize + dx) >= grid.len() as isize
                //     || (vy as isize + dy) >= grid[0].len() as isize
                //     || (vx as isize + dx) < 0
                //     || (vy as isize + dy) < 0
                // {
                //     //todo
                //     continue;
                // }
                // let nbr = grid[((vx as isize) + dx) as usize][((vy as isize) + dy) as usize];
                // let vx = vx as isize;
                // let vy = vy as isize;
                match (grid[vx as usize][vy as usize], (dx, dy)) {
                    ('.', (_, _)) => {
                        if !seen.contains_key(&(vx + dx, vy + dy, dx, dy)) {
                            to_visit.push_back((vx + dx, vy + dy, (dx, dy)))
                        }
                    }
                    // ('/', ()) //UP GOES CW, RIGhT GOES ANTICW, LEFT GOES ANTICW, DOWN GOES CW
                    ('/', (a, b)) => {
                        let (mut ndx, mut ndy) = (0, 0);
                        (ndx, ndy) = match (a, b) {
                            (0, 1) => (-1, 0), //BEAM GOING RIGHT GOES UP
                            (0, -1) => (1, 0), //BEAM GOING LEFT GOES DOWN
                            (1, 0) => (0, -1),
                            (-1, 0) => (0, 1),
                            _ => panic!("illegal"),
                        };
                        // if a != 0 {
                        //     //GOING UP OR DOWN
                        //     (ndx, ndy) = rot(dx, dy, true);
                        // } else {
                        //     (ndx, ndy) = rot(dx, dy, false);
                        // }
                        println!("dx after rot at right mirror = {ndx} {ndy}");

                        if !seen.contains_key(&(vx + ndx, vy + ndy, ndx, ndy)) {
                            to_visit.push_back((vx + ndx, vy + ndy, (ndx, ndy)));
                        }
                    }
                    //UP OGES ANTICW, DOWN GOES ANTI CW
                    ('\\', (a, b)) => {
                        let (ndx, ndy) = match (a, b) {
                            (0, 1) => (1, 0),
                            (0, -1) => (-1, 0),
                            (1, 0) => (0, 1),
                            (-1, 0) => (0, -1),
                            _ => panic!("illegal"),
                        };

                        println!("SAW \\ at {vx} {vy} going {ndx} {ndy}");
                        // if b != 0 {
                        // (ndx, ndy) = rot(dx, dy, true);
                        // } else {
                        // (ndx, ndy) = rot(dx, dy, false);
                        // }

                        // let (ndx, ndy) = rot(dx, dy, false);
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

            // println!("FINISHED SEEN\n={seen:?}");
            let mut total = 0;

            // println!("seen\n{seen:?}");
            let mut seen2 = vec![vec![false; grid[0].len()]; grid.len()];
            for ((x, y, dx, dy), b) in seen {
                if b {
                    seen2[x as usize][y as usize] = true;
                }
            }
            // grid::print_grid(&seen2);
            for row in seen2 {
                for item in row {
                    if item {
                        // print!("#");
                        total += 1;
                    } else {
                        // print!(".")
                    }
                    // if item {
                    // }
                }
                // println!();
            }
            best = best.max(total);
        }

        println!("Best so far = {best}");
    }

    best

    // todo!()
}
