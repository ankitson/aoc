use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use util::grid::*;

pub type Input = (Vec<Vec<char>>, (isize, isize), usize, usize);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let mut grid = vec![];
    let mut start = (0isize, 0isize);
    let (mut nr, mut nc) = (0, 0);
    for (rownum, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (colnum, char) in line.chars().enumerate() {
            row.push(char);
            if "^>v<".contains(char) {
                start = (rownum.try_into().unwrap(), colnum.try_into().unwrap())
            }
            nc = nc.max(colnum);
        }
        grid.push(row);
        nr = nr.max(rownum);
    }
    (grid, start, nr + 1, nc + 1)
}

fn arrow_to_dir(ch: char) -> (isize, isize) {
    return match ch {
        '^' => (-1isize, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1isize),
        _ => panic!("illegal dir"),
    };
}

fn rot((dr, dc): (isize, isize)) -> (isize, isize) {
    return (dc, -dr); //shortcut
}

pub fn tour_grid(
    grid: &Vec<Vec<char>>,
    mut cr: isize,
    mut cc: isize,
    mut dr: isize,
    mut dc: isize,
) -> HashSet<(isize, isize)> {
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter([(cr, cc)].into_iter());
    while inbounds2z((cr, cc), grid.len() as isize, grid[0].len() as isize) {
        visited.insert((cr, cc));
        let (nr, nc) = (cr + dr, cc + dc);
        if !inbounds2z((nr, nc), grid.len() as isize, grid[0].len() as isize) {
            break;
        }
        let next = grid[nr as usize][nc as usize];
        if next == '.' {
            (cr, cc) = (nr, nc);
            continue;
        } else if next == '#' {
            (dr, dc) = rot((dr, dc))
        } else {
            panic!("illegal grid char")
        }
    }
    visited
}

pub fn part1(raw_input: &str) -> Output {
    let (mut grid, (sr, sc), _, _) = parse(raw_input);
    let (dr, dc) = arrow_to_dir(grid[sr as usize][sc as usize]);
    grid[sr as usize][sc as usize] = '.';
    let visited_locs = tour_grid(&grid, sr, sc, dr, dc);
    visited_locs.len()
}

//bruteforce
pub fn part2(raw_input: &str) -> Output {
    let (mut grid, (sr, sc), _, _) = parse(raw_input);
    let (mut dr, mut dc) = arrow_to_dir(grid[sr as usize][sc as usize]);
    let (sdr, sdc) = (dr, dc);

    grid[sr as usize][sc as usize] = '.';
    let visited_locs = tour_grid(&grid, sr, sc, dr, dc);
    let mut causes_loop = HashSet::new();
    for (rownum, colnum) in visited_locs {
        if (rownum, colnum) != (sr, sc) && grid[sr as usize][sc as usize] == '.' {
            grid[rownum as usize][colnum as usize] = '#';
            let (mut cr, mut cc) = (sr, sc);
            (dr, dc) = (sdr, sdc);
            let mut visited = HashSet::new();
            while inbounds2z((cr, cc), grid.len() as isize, grid[0].len() as isize) {
                if visited.contains(&(cr, cc, dr, dc)) {
                    causes_loop.insert((rownum, colnum));
                    break;
                }
                visited.insert((cr, cc, dr, dc));
                let (nr, nc) = (cr + dr, cc + dc);
                if !inbounds2z((nr, nc), grid.len() as isize, grid[0].len() as isize) {
                    break;
                }
                let next = grid[nr as usize][nc as usize];
                if next == '.' {
                    (cr, cc) = (nr, nc);
                    continue;
                } else if next == '#' {
                    (dr, dc) = rot((dr, dc))
                } else {
                    panic!("illegal grid char")
                }
            }
            grid[rownum as usize][colnum as usize] = '.';
        }
    }
    causes_loop.len()
}

//it only catches cycles if i immediately enter them after turning right, but not later
// pub fn part2_wrong(raw_input: &str) -> Output {
//     let (mut grid_map, (sr, sc), _, _) = parse(raw_input);
//     let (mut cr, mut cc) = (sr, sc);
//     let (mut dr, mut dc) = match (grid_map.get(&(cr, cc)).unwrap()) {
//         '^' => (-1isize, 0),
//         '>' => (0, 1),
//         'v' => (1, 0),
//         '<' => (0, -1isize),
//         _ => panic!("illegal dir"),
//     };

//     let mut visited = HashSet::new();
//     let mut last_loc = (cr, cc);
//     grid_map.entry((cr, cc)).insert_entry('.');
//     let mut causes_loop = HashSet::new();
//     while grid_map.contains_key(&(cr, cc)) {
//         visited.insert((cr, cc, dr, dc));
//         let (nr, nc) = (cr + dr, cc + dc);
//         if !grid_map.contains_key(&(nr, nc)) {
//             break;
//         }
//         let at_loc = grid_map.get(&(nr, nc)).unwrap();
//         if *at_loc == '.' {
//             let (cdr, cdc) = rot((dr, dc));
//             if visited.contains(&(cr + cdr, cc + cdc, cdr, cdc)) && last_loc != (cr + cdr, cc + cdc) {
//                 println!("at {cr},{cc} facing {dr},{dc}, having visited = {visited:?}\nif i turn right here, i will visit already seen {:?} ", (cr+cdr, cc+cdc));
//                 causes_loop.insert((nr, nc));
//             }

//             last_loc = (cr, cc);
//             (cr, cc) = (nr, nc);
//             continue;
//         } else if *at_loc == '#' {
//             (dr, dc) = rot((dr, dc))
//         } else {
//             panic!("illegal grid char")
//         }
//     }
//     println!("causes loop = {:?}", causes_loop);
//     causes_loop.len()
// }
