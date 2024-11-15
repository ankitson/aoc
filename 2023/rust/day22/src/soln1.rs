use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use regex::Regex;

pub type U3 = (usize, usize, usize);
pub type Input = Vec<(U3, U3)>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let bricks = input
        .lines()
        .map(|line| {
            let (starts, ends) = line.split_once("~").unwrap();
            let scs = starts.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
            let (sx, sy, sz) = (scs[0], scs[1], scs[2]);
            let ecs = ends.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
            let (ex, ey, ez) = (ecs[0], ecs[1], ecs[2]);
            ((sx, sy, sz), (ex, ey, ez))
        })
        .collect_vec();
    bricks
}

fn dropped(tallest: &HashMap<(usize, usize), usize>, brick: &mut (U3, U3)) -> usize {
    let peak = *(brick.0 .0..=brick.1 .0)
        .map(|x| (brick.0 .1..=brick.1 .1).map(|y| tallest.get(&(x, y)).unwrap_or(&0)).max().unwrap_or(&0))
        .max()
        .unwrap_or(&0);
    let fall = (brick.0 .2 as i32 - peak as i32 - 1).max(0) as usize;
    brick.0 .2 -= fall as usize;
    brick.1 .2 -= fall as usize;
    fall
}

fn fall_all(bricks: &mut Vec<(U3, U3)>) -> usize {
    let mut tallest = HashMap::new();
    let mut falls = 0;
    for i in 0..bricks.len() {
        let f = dropped(&tallest, &mut bricks[i]);
        falls += if f > 0 { 1 } else { 0 };
        for x in bricks[i].0 .0..=bricks[i].1 .0 {
            for y in bricks[i].0 .1..=bricks[i].1 .1 {
                tallest.insert((x, y), bricks[i].1 .2);
            }
        }
    }
    falls
}

pub fn part1(raw_input: &str) -> Output {
    let mut bricks = parse(raw_input);
    bricks.sort_by_key(|((_, _, z1), (_, _, _))| *z1);
    fall_all(&mut bricks);

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut bricks_without = bricks.clone();
        bricks_without.remove(i);
        let nfalls = fall_all(&mut bricks_without);
        total += if nfalls == 0 { 1 } else { 0 };
    }
    total
}

pub fn part2(raw_input: &str) -> Output {
    let mut bricks = parse(raw_input);
    bricks.sort_by_key(|((_, _, z1), (_, _, _))| *z1);
    fall_all(&mut bricks);

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut bricks_without = bricks.clone();
        bricks_without.remove(i);
        let nfalls = fall_all(&mut bricks_without);
        total += nfalls;
    }
    total
}
