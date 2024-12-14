use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};
use util::grid::*;

type NUM = i64;
pub type Input = HashMap<usize, ((NUM, NUM), (NUM, NUM))>;
pub type Output = NUM;

pub fn parse(input: &str) -> Input {
    // let mut grid = vec![];
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
        // let re = Regex::new(r"(-)?\d+").unwrap();
        // let nums: Vec<NUM> = re.captures_iter(line).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect_vec();
        // robots.insert(idx, ((nums[0], nums[1]), (nums[2], nums[3])));
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

pub fn part1(raw_input: &str) -> Output {
    const NUM_COLS: NUM = 101;
    const NUM_ROWS: NUM = 103;
    let robots_map = parse(raw_input);
    let mut qdrants = vec![0, 0, 0, 0];
    let mut final_poses = HashSet::new();
    for (robot_num, ((col, row), (vel_col, vel_row))) in robots_map.iter() {
        let (mut fcol, mut frow) = (*col, *row);
        for incr in 0..10000 {
            final_poses.insert((frow as usize, fcol as usize));
            // println!("poses = {final_poses:?} coord = {fcol},{frow} vcol = {vel_col} vrow = {vel_row}");
            // println!("BEFORE SECOND {incr}");
            // print_grid_spcl_locs(&final_poses, NUM_ROWS as usize, NUM_COLS as usize);
            // println!("-----------------");
            final_poses.remove(&(frow as usize, fcol as usize));
            fcol = next(fcol, *vel_col, NUM_COLS) as i64;
            frow = next(frow, *vel_row, NUM_ROWS) as i64;
        }
        // println!("robot {robot_num} pos = {frow} {fcol}");
        final_poses.insert((frow as usize, fcol as usize));
        let final_quad = quad(fcol, frow, NUM_COLS, NUM_ROWS);
        if final_quad != 10 {
            qdrants[final_quad] += 1;
        }
    }

    print_grid_spcl_locs(&final_poses, NUM_ROWS as usize, NUM_COLS as usize);
    // pub fn print_grid_spcl_locs<U>(locs: &HashSet<(usize, usize), U>, num_rows: usize, num_cols: usize) {

    println!("quads = {qdrants:?}");
    qdrants.iter().product()
}

pub fn part2(raw_input: &str) -> Output {
    const NUM_COLS: NUM = 101;
    const NUM_ROWS: NUM = 103;
    let robots_map = parse(raw_input);
    let mut qdrants = vec![0, 0, 0, 0];
    let mut final_poses = HashSet::new();

    // At start of program
    // print!("\x1B[?1049h"); // Enter alternate buffer
    // let mut poses = HashMap::new();
    // for (robot_num, ((col, row), _)) in &robots_map {
    // poses.insert(robot_num, (*col, *row));
    // }
    let mut poses = vec![(0, 0); robots_map.len()];
    for (robot_num, ((col, row), _)) in &robots_map {
        poses[*robot_num] = (*col, *row);
    }

    for incr in 0..20000 {
        // Clear screen and move cursor to top-left
        // print!("\x1B[2J\x1B[1;1H");
        for (robot_num, ((col, row), (vel_col, vel_row))) in robots_map.iter() {
            let (mut fcol, mut frow) = poses[*robot_num];

            // final_poses.insert((frow as usize, fcol as usize));
            // println!("poses = {final_poses:?} coord = {fcol},{frow} vcol = {vel_col} vrow = {vel_row}");
            // println!("BEFORE SECOND {incr}");
            // print_grid_spcl_locs(&final_poses, NUM_ROWS as usize, NUM_COLS as usize);
            // println!("-----------------");
            // final_poses.remove(&(frow as usize, fcol as usize));
            fcol = next(fcol, *vel_col, NUM_COLS) as i64;
            frow = next(frow, *vel_row, NUM_ROWS) as i64;
            poses[*robot_num] = (fcol, frow);
            final_poses.insert((frow as usize, fcol as usize));
            // let final_quad = quad(fcol, frow, NUM_COLS, NUM_ROWS);
        }
        let sec = incr + 1;
        println!("AFTER SECOND {sec}"); // poses = {final_poses:?}");
        print_grid_spcl_locs(&final_poses, NUM_ROWS as usize, NUM_COLS as usize);
        final_poses.clear();
        // println!("robot {robot_num} pos = {frow} {fcol}");
        // if final_quad != 10 {
        // qdrants[final_quad] += 1;
        // }
        // thread::sleep(Duration::from_millis(100));
    }

    // print!("\x1B[2J\x1B[1;1H");

    print_grid_spcl_locs(&final_poses, NUM_ROWS as usize, NUM_COLS as usize);
    // pub fn print_grid_spcl_locs<U>(locs: &HashSet<(usize, usize), U>, num_rows: usize, num_cols: usize) {

    print!("\x1B[?1049l"); // Exit alternate buffer

    println!("quads = {qdrants:?}");
    qdrants.iter().product()
}
