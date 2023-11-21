use std::{
    any::{Any, TypeId},
    default,
    fmt::{Debug, Display},
};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

pub type Input = OffsetGrid<bool>;
pub type Output = String;

#[derive(Debug, Clone)]
pub struct OffsetGrid<T: Clone> {
    grid: Vec<Vec<T>>,
    offset: (isize, isize),
}
impl<T: Any + Clone + Debug> Display for OffsetGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OffsetGrid(\n")?;
        for row in &self.grid {
            f.write_str("[")?;
            for item in row {
                if TypeId::of::<T>() == TypeId::of::<bool>() {
                    if let Some(&b) = (item as &dyn Any).downcast_ref::<bool>() {
                        f.write_fmt(format_args!("{}", if b { "." } else { "#" }))?;
                    }
                } else {
                    f.write_fmt(format_args!("{:?},", item))?;
                }
            }
            f.write_str("]\n")?;
        }
        f.write_fmt(format_args!(",offset={:?})", self.offset))
    }
}

impl<T: Clone + Debug> OffsetGrid<T> {
    fn new_with_default(size: usize, offset: (isize, isize), default: T) -> OffsetGrid<T> {
        OffsetGrid { grid: vec![vec![default; size]; size], offset: offset }
    }
    fn transform(&self, (x, y): (usize, usize)) -> (usize, usize) {
        let real_x = ((((x as isize) + self.offset.0) + (self.grid.len() as isize)) as usize) % self.grid.len();
        let real_y = ((((y as isize) + self.offset.1) + (self.grid.len() as isize)) as usize) % self.grid.len();
        (real_x, real_y)
    }
    fn get(&self, (x, y): (usize, usize)) -> &T {
        let (rx, ry) = self.transform((x, y));
        &self.grid[rx][ry]
    }

    fn set(&mut self, (x, y): (usize, usize), v: T) {
        let (rx, ry) = self.transform((x, y));
        self.grid[rx][ry] = v;
    }
}
impl<T: Default + Clone> OffsetGrid<T> {
    fn new(size: usize, offset: (isize, isize)) -> OffsetGrid<T> {
        OffsetGrid { grid: vec![vec![T::default(); size]; size], offset: offset }
    }
}

pub fn parse(input: &str) -> Input {
    let rock_coords = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cs| {
                    cs.split_once(",").map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())).unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut og = OffsetGrid::new_with_default(10, (0, -474), true);
    for rock in rock_coords {
        for pair in rock.windows(2) {
            let (c1x, c1y) = pair[0];
            let (c2x, c2y) = pair[1];
            if c1x == c2x {
                for y in c1y.min(c2y)..=c1y.max(c2y) {
                    og.set((y, c1x), false);
                }
            } else if c1y == c2y {
                for x in c1x.min(c2x)..=c1x.max(c2x) {
                    og.set((c1y, x), false);
                }
            } else {
                panic!("illegal input")
            }
        }
    }
    og
}
