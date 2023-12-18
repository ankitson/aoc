use std::{
    collections::{HashSet, VecDeque},
    f64::consts::PI,
};

use itertools::Itertools;
use regex::Regex;

pub type Input = String;
pub type Output = usize;

pub fn parse(input: &str) -> Vec<(&str, usize)> {
    let lines = input
        .lines()
        .map(|line| {
            let mut splits = line.split_ascii_whitespace();
            let dir = splits.next().unwrap();
            let dist = splits.next().unwrap().parse::<usize>().unwrap();

            (dir, dist)
        })
        .collect_vec();
    lines
}

pub fn parse2(input: &str) -> Vec<(&str, usize)> {
    let lines = input
        .lines()
        .map(|line| {
            let mut splits = line.split_ascii_whitespace();
            let dir = splits.next().unwrap();
            let dist = splits.next().unwrap().parse::<usize>().unwrap();
            let color = splits.next().unwrap();
            let color = &color[2..color.len() - 1];
            // println!("color = {color}");
            let colord = &color[0..color.len() - 1];
            let dist = usize::from_str_radix(colord, 16).unwrap();
            let dir = match color.chars().last().unwrap() {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!(),
            };
            (dir, dist)
        })
        .collect_vec();
    lines
}

fn dir(d: &str) -> (isize, isize) {
    match d.chars().nth(0).unwrap() {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (-1, 0),
        'D' => (1, 0),
        _ => panic!(""),
    }
}

pub fn print_grid(grid: &Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let ch = if (grid[i][j]) { "#" } else { "." };
            print!("{}", ch);
        }
        println!("");
    }
}

fn count(grid: &Vec<Vec<bool>>, sr: isize, sc: isize, seen: &mut HashSet<(isize, isize)>) -> i32 {
    if sr < 0
        || sr >= grid.len() as isize
        || sc < 0
        || sc >= grid[0].len() as isize
        || grid[sr as usize][sc as usize]
        || seen.contains(&(sr, sc))
    {
        return 0;
    }
    let gval = grid[sr as usize][sc as usize];
    println!("count at {sr} {sc} = {gval}");
    let mut counter = 1;
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nr = sr + dx;
        let ny = sc + dy;
        // let mut seen_new = seen.clone();
        seen.insert((sr, sc));
        counter += count(grid, nr, ny, seen);
    }

    counter
}

fn count_bfs(grid: &Vec<Vec<bool>>, sr: isize, sc: isize) -> usize {
    let mut to_visit = VecDeque::new();
    let mut seen = HashSet::new();
    let mut count = 0;
    to_visit.push_back((sr, sc));
    while to_visit.len() > 0 {
        let (vr, vc) = to_visit.pop_front().unwrap();
        if !seen.contains(&(vr, vc)) {
            // println!("BFS VISIT {vr} {vc}");
            count += 1;
            seen.insert((vr, vc));
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = vr + dx;
                let nc = vc + dy;
                // println!("nr = {nr} nc = {nc}");
                if nr >= 0
                    && nr < grid.len() as isize
                    && nc >= 0
                    && nc < grid[0].len() as isize
                    && !seen.contains(&(nr, nc))
                    && !grid[nr as usize][nc as usize]
                {
                    // println!("add nbr {nr} {nc}");
                    to_visit.push_back((nr, nc));
                }
            }
        }
    }
    count
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 2000]; 2000];
    let mut sr = 1000;
    let mut sc = 1000;
    let mut boundary_points = 0;
    let mut ir = sr - 1;
    let mut ic = sc - 1;
    for (diri, dist) in input {
        let (dx, dy) = dir(diri);
        if dx == 0 && dy == 1 && ir == 0 && ic == 0 && !grid[(sr as usize) + 1][(sc as usize)] {
            ir = sr + 1;
            ic = sc;
        }
        let nsr = sr + (dx) * dist as isize;
        let nsc = sc + (dy) * dist as isize;
        for i in (sr.min(nsr)).max(0)..=(sr.max(nsr)).min(grid.len() as isize) {
            println!(" i = {i} sr ={sr} nsr = {nsr} sc = {sc}");
            if !grid[i as usize][sc as usize] {
                boundary_points += 1;
            }
            grid[i as usize][sc as usize] = true;
        }
        for j in sc.min(nsc).max(0)..=sc.max(nsc).min(grid[0].len() as isize) {
            if !grid[sr as usize][j as usize] {
                boundary_points += 1;
            }
            grid[sr as usize][j as usize] = true;
        }
        sr = nsr;
        sc = nsc;
    }
    // println!("here");
    // print_grid(&grid);

    // let mut seen = HashSet::new();
    // let interior_count = count(&grid, 1, 3, &mut seen);
    let mut interior_count = count_bfs(&grid, ir, ic);
    let count = interior_count + boundary_points;
    println!("COUNT = {count} boundary_pts = {boundary_points} interior = {interior_count}");
    count as usize
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse2(raw_input);
    // let mut grid: Vec<Vec<bool>> = vec![vec![false; 200000]; 200000];
    let mut sr = 100000;
    let mut sc = 100000;
    let mut boundary_points = 0;
    // let mut ir = sr - 1;
    // let mut ic = sc - 1;

    // let mut all_points = HashSet::new();
    let mut area = 0;
    // let mut points_vec = vec![];
    for (diri, dist) in input {
        let (dx, dy) = dir(diri);

        let nsr = sr + (dx) * dist as isize;
        let nsc = sc + (dy) * dist as isize;

        area += (sc * nsr) - (nsc * sr);

        boundary_points += dist;
        // for i in (sr.min(nsr))..=(sr.max(nsr)) {
        //     // println!(" i = {i} sr ={sr} nsr = {nsr} sc = {sc}");
        //     if !all_points.contains(&(i as usize, sc as usize)) {
        //         all_points.insert((i as usize, sc as usize));
        //         points_vec.push((i as usize, sc as usize));
        //         boundary_points += 1;
        //     }
        //     // grid[i as usize][sc as usize] = true;
        // }
        // for j in sc.min(nsc)..=sc.max(nsc) {
        //     if !all_points.contains(&(sr as usize, j as usize)) {
        //         all_points.insert((sr as usize, j as usize));
        //         points_vec.push((sr as usize, j as usize));
        //         // all_points.push((sr as usize, j as usize));
        //         boundary_points += 1;
        //     }
        //     // grid[sr as usize][j as usize] = true;
        // }
        sr = nsr;
        sc = nsc;
    }

    // fn atan_360(y: f64, x: f64) -> f64 {
    //     let phi = (y as f64).atan2(x as f64);
    //     let phi2 = if phi < 0.0 { phi + 2.0 * PI } else { phi };
    //     phi2
    // }

    // let mut all_points = all_points.iter().collect_vec();
    // all_points.sort_by_key(|(r, c)| {
    //     let x = c;
    //     let y = r;
    //     // let r = (x.pow(2) as f64 + y.pow(2) as f64).sqrt();
    //     let phi = (*y as f64).atan2(*x as f64);
    //     let phi2 = if phi < 0.0 { phi + 2.0 * PI } else { phi };
    //     phi2 as isize
    // });

    // let p1 = atan_360(1f64, 1f64);
    // let p2 = atan_360(0f64, 1f64);
    // let p3 = atan_360(-1f64, 1f64);
    // let p4 = atan_360(-1f64, -1f64);
    // let p5 = atan_360(0f64, -1f64);
    // let p6 = atan_360(1f64, -1f64);
    // let p7 = atan_360(1f64, 0f64);

    // let ps = vec![p1, p2, p3, p4, p5, p6, p7];
    // println!("ps: {ps:?}");
    // println!("all points = {all_points:?}");

    // let mut add_term = 0;
    // let mut sub_term = 0;
    // // points_vec.reverse();
    // println!("points: {points_vec:?}");
    // for window in points_vec.windows(2) {
    //     let (y1, x1) = window[0];
    //     let (y2, x2) = window[1];
    //     add_term += (x1 * y2);
    //     sub_term += (y1 * x2);
    // }
    // let (firstx, firsty) = points_vec.first().unwrap();
    // let (lastx, lasty) = points_vec.last().unwrap();
    // add_term += (lastx * firsty);
    // sub_term += (lasty * firstx);

    // let area = (0.5 * ((add_term - sub_term) as f64)) as usize;
    let total = (area.abs() / 2) as usize + boundary_points / 2 + 1;
    println!("AREA = {area} boundary = {boundary_points} total = {total}");
    // println!("boundary = {boundary_points}");
    total as usize
}

//976171 with 500x500
