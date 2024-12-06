use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

pub type Input = (HashMap<(isize, isize), char>, (isize, isize));
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let mut grid = HashMap::new();
    let mut start = (0isize, 0isize);
    for (rownum, line) in input.lines().enumerate() {
        for (colnum, char) in line.chars().enumerate() {
            grid.entry((rownum as isize, colnum as isize)).or_insert(char);
            if "^>v<".contains(char) {
                start = (rownum.try_into().unwrap(), colnum.try_into().unwrap())
            }
        }
    }
    (grid, start)
}

fn rot((dr, dc): (isize, isize)) -> (isize, isize) {
    match (dr, dc) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!("illegal dir"),
    }
}

pub fn part1(raw_input: &str) -> Output {
    let (mut grid_map, (sr, sc)) = parse(raw_input);
    let mut visited = 0;
    let (mut cr, mut cc) = (sr, sc);
    let (mut dr, mut dc) = match (grid_map.get(&(cr, cc)).unwrap()) {
        '^' => (-1isize, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1isize),
        _ => panic!("illegal dir"),
    };

    let mut visited = HashSet::new();
    visited.insert((cr, cc));
    grid_map.entry((cr, cc)).insert_entry('.');
    while grid_map.contains_key(&(cr, cc)) {
        visited.insert((cr, cc));
        let (nr, nc) = (cr + dr, cc + dc);
        if !grid_map.contains_key(&(nr, nc)) {
            break;
        }
        let at_loc = grid_map.get(&(nr, nc)).unwrap();
        if *at_loc == '.' {
            (cr, cc) = (nr, nc);
            continue;
        } else if *at_loc == '#' {
            (dr, dc) = rot((dr, dc))
        } else {
            panic!("illegal grid char")
        }
    }

    visited.len()
}

//(7,4) is extra not in answer but in mine
//(8,1) is missed in answer but not in mine
pub fn part2(raw_input: &str) -> Output {
    let (mut grid_map, (sr, sc)) = parse(raw_input);
    let (mut cr, mut cc) = (sr, sc);
    let (mut dr, mut dc) = match (grid_map.get(&(cr, cc)).unwrap()) {
        '^' => (-1isize, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1isize),
        _ => panic!("illegal dir"),
    };

    let mut visited = HashSet::new();
    visited.insert((cr, cc));
    let mut last_loc = (cr, cc);
    grid_map.entry((cr, cc)).insert_entry('.');
    let mut causes_loop = HashSet::new();
    while grid_map.contains_key(&(cr, cc)) {
        visited.insert((cr, cc));
        let (nr, nc) = (cr + dr, cc + dc);
        if !grid_map.contains_key(&(nr, nc)) {
            break;
        }
        let at_loc = grid_map.get(&(nr, nc)).unwrap();
        if *at_loc == '.' {
            let (cdr, cdc) = rot((dr, dc));
            if visited.contains(&(cr + cdr, cc + cdc)) && last_loc != (cr + cdr, cc + cdc) {
                causes_loop.insert((nr, nc));
            }

            last_loc = (cr, cc);
            (cr, cc) = (nr, nc);
            continue;
        } else if *at_loc == '#' {
            (dr, dc) = rot((dr, dc))
        } else {
            panic!("illegal grid char")
        }
    }
    println!("causes loop = {:?}", causes_loop);
    causes_loop.len()
}

pub fn part2_bf(raw_input: &str) -> Output {
    let (mut grid_map, (sr, sc)) = parse(raw_input);
    let (mut cr, mut cc) = (sr, sc);
    let (mut dr, mut dc) = match (grid_map.get(&(cr, cc)).unwrap()) {
        '^' => (-1isize, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1isize),
        _ => panic!("illegal dir"),
    };
    let (sdr, sdc) = (dr, dc);

    // let mut last_loc = (cr, cc);
    grid_map.entry((cr, cc)).insert_entry('.');

    let mut causes_loop = HashSet::new();
    for rownum in 0..130 {
        println!("start row {:?}", rownum);
        for colnum in 0..130 {
            if (rownum, colnum) != (sr, sc) && *grid_map.get(&(rownum, colnum)).unwrap() == '.' {
                grid_map.entry((rownum, colnum)).insert_entry('#');
                (cr, cc) = (sr, sc);
                (dr, dc) = (sdr, sdc);
                let mut visited = HashSet::new();
                'outer: while grid_map.contains_key(&(cr, cc)) {
                    if visited.contains(&(cr, cc, dr, dc)) {
                        causes_loop.insert((rownum, colnum));
                        break 'outer;
                    }
                    visited.insert((cr, cc, dr, dc));
                    let (nr, nc) = (cr + dr, cc + dc);
                    if !grid_map.contains_key(&(nr, nc)) {
                        break;
                    }
                    let at_loc = grid_map.get(&(nr, nc)).unwrap();
                    if *at_loc == '.' {
                        (cr, cc) = (nr, nc);
                        continue;
                    } else if *at_loc == '#' {
                        (dr, dc) = rot((dr, dc))
                    } else {
                        panic!("illegal grid char")
                    }
                }
                grid_map.entry((rownum, colnum)).insert_entry('.');
            }
        }
    }

    println!("causes loop = {:?}", causes_loop);
    causes_loop.len()
}
