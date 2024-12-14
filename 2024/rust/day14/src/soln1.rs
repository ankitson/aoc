use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
    thread,
    time::Duration,
};
use util::grid::*;

type NUM = i64;
pub type Input = HashMap<usize, ((NUM, NUM), (NUM, NUM))>;
pub type Output = NUM;

pub fn parse(input: &str) -> Input {
    let mut robots: HashMap<usize, ((NUM, NUM), (NUM, NUM))> = HashMap::new();
    for (idx, line) in input.lines().enumerate() {
        let (posstr, vstr) = line.split_once(" ").unwrap();
        let (_, posstr) = posstr.split_once("=").unwrap();
        let (p1, p2) = posstr.split_once(",").unwrap();
        let p1 = p1.parse().unwrap();
        let p2 = p2.parse().unwrap();
        let (_, vstr) = vstr.split_once("=").unwrap();
        let (v1, v2) = vstr.split_once(",").unwrap();
        let v1 = v1.parse().unwrap();
        let v2 = v2.parse().unwrap();
        robots.insert(idx, ((p1, p2), (v1, v2)));
    }

    robots
}

fn next(start_x: NUM, incr_x: NUM, bounds: NUM) -> u64 {
    ((((start_x + incr_x) % bounds) + bounds) % bounds) as u64
}

fn quad(col: NUM, row: NUM, num_cols: NUM, num_rows: NUM) -> usize {
    if col == num_cols / 2 || row == num_rows / 2 {
        return 10;
    }
    if col < num_cols / 2 {
        if row < num_rows / 2 {
            0
        } else {
            2
        }
    } else {
        if row < num_rows / 2 {
            1
        } else {
            3
        }
    }
}

pub fn part1(raw_input: &str, num_rows: NUM, num_cols: NUM) -> Output {
    let robots_map = parse(raw_input);
    let mut qdrants = vec![0, 0, 0, 0];
    let mut final_poses = HashSet::new();
    for (robot_num, ((col, row), (vel_col, vel_row))) in robots_map.iter() {
        let (mut fcol, mut frow) = (*col, *row);
        for incr in 0..100 {
            final_poses.insert((frow as usize, fcol as usize));
            final_poses.remove(&(frow as usize, fcol as usize));
            fcol = next(fcol, *vel_col, num_cols) as i64;
            frow = next(frow, *vel_row, num_rows) as i64;
        }
        final_poses.insert((frow as usize, fcol as usize));
        let final_quad = quad(fcol, frow, num_cols, num_rows);
        if final_quad != 10 {
            qdrants[final_quad] += 1;
        }
    }

    print_grid_spcl_locs(&final_poses, num_rows as usize, num_cols as usize);
    qdrants.iter().product()
}

pub fn part2(raw_input: &str, num_rows: NUM, num_cols: NUM) {
    let robots_map = parse(raw_input);
    let mut poses = vec![(0, 0); robots_map.len()];
    for (robot_num, ((col, row), _)) in &robots_map {
        poses[*robot_num] = (*col, *row);
    }

    for incr in 0..10000 {
        for (robot_num, ((col, row), (vel_col, vel_row))) in robots_map.iter() {
            let (mut fcol, mut frow) = poses[*robot_num];
            fcol = next(fcol, *vel_col, num_cols) as i64;
            frow = next(frow, *vel_row, num_rows) as i64;
            poses[*robot_num] = (fcol, frow);
        }
        let sec = incr + 1;
        println!("AFTER SECOND {sec}");
        let pos_set: HashSet<_, RandomState> = HashSet::from_iter(poses.iter().map(|v| (v.1 as usize, v.0 as usize)));
        print_grid_spcl_locs(&pos_set, num_rows as usize, num_cols as usize);
    }
}
