use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

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
            // grid.entry((rownum as isize, colnum as isize)).or_insert(char);
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

#[inline]
fn inbounds1<T: PartialOrd>(x: T, min: T, max: T) -> bool {
    return x >= min && x < max;
}

#[inline]
fn inbounds2<T: PartialOrd>((x, y): (T, T), (min1, max1): (T, T), (min2, max2): (T, T)) -> bool {
    return inbounds1(x, min1, max1) && inbounds1(y, min2, max2);
}

#[inline]
fn inbounds2z<T: PartialOrd + From<u8>>((x, y): (T, T), max1: T, max2: T) -> bool {
    return inbounds2((x, y), (T::from(0u8), max1), (T::from(0u8), max2));
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
    grid[sr as usize][sc as usize] = '.';
    let visited_locs = tour_grid(&grid, sr, sc, dr, dc);

    let (mut cr, mut cc) = (sr, sc);
    let (sdr, sdc) = (dr, dc);

    let mut causes_loop = HashSet::new();
    for (rownum, colnum) in visited_locs {
        if (rownum, colnum) != (sr, sc) && grid[sr as usize][sc as usize] == '.' {
            grid[rownum as usize][colnum as usize] = '#';
            (cr, cc) = (sr, sc);
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

// pub fn part2_par(raw_input: &str) -> Output {
//     let (mut grid_map, (sr, sc), numrows, numcols) = parse(raw_input);
//     let (mut cr, mut cc) = (sr, sc);
//     let (mut dr, mut dc) = match (grid_map.get(&(cr, cc)).unwrap()) {
//         '^' => (-1isize, 0),
//         '>' => (0, 1),
//         'v' => (1, 0),
//         '<' => (0, -1isize),
//         _ => panic!("illegal dir"),
//     };
//     let (sdr, sdc) = (dr, dc);

//     grid_map.entry((cr, cc)).insert_entry('.');
//     let mut causes_loop = HashSet::new();
//     (0..numrows.try_into().unwrap()).into_par_iter().for_each(|rownum| {
//         for colnum in 0..numcols.try_into().unwrap() {
//             if (rownum, colnum) != (sr, sc) && *grid_map.get(&(rownum, colnum)).unwrap() == '.' {
//                 grid_map.entry((rownum, colnum)).insert_entry('#');
//                 (cr, cc) = (sr, sc);
//                 (dr, dc) = (sdr, sdc);
//                 let mut visited = HashSet::new();
//                 'outer: while grid_map.contains_key(&(cr, cc)) {
//                     if visited.contains(&(cr, cc, dr, dc)) {
//                         causes_loop.insert((rownum, colnum));
//                         break 'outer;
//                     }
//                     visited.insert((cr, cc, dr, dc));
//                     let (nr, nc) = (cr + dr, cc + dc);
//                     if !grid_map.contains_key(&(nr, nc)) {
//                         break;
//                     }
//                     let at_loc = grid_map.get(&(nr, nc)).unwrap();
//                     if *at_loc == '.' {
//                         (cr, cc) = (nr, nc);
//                         continue;
//                     } else if *at_loc == '#' {
//                         (dr, dc) = rot((dr, dc))
//                     } else {
//                         panic!("illegal grid char")
//                     }
//                 }
//                 grid_map.entry((rownum, colnum)).insert_entry('.');
//             }
//         }
//     });
//     causes_loop.len()
// }

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
