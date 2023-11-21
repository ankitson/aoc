use std::{
    any::{Any, TypeId},
    fmt::{Debug, Display},
};

use itertools::Itertools;

pub type Input = OffsetGrid<bool>;
pub type Output = usize;

#[derive(Debug, Clone)]
pub struct OffsetGrid<T: Clone> {
    pub grid: Vec<Vec<T>>,
    pub offset: (isize, isize),
    pub wrap: bool,
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
    pub fn new_with_default(size: usize, offset: (isize, isize), default: T, wrap: bool) -> OffsetGrid<T> {
        OffsetGrid { grid: vec![vec![default; size]; size], offset: offset, wrap: wrap }
    }
    pub fn transform(&self, (x, y): (isize, isize)) -> (usize, usize) {
        let mut real_x = (((x as isize) + self.offset.0) as usize);
        let mut real_y = (((y as isize) + self.offset.1) as usize);
        if self.wrap {
            real_x = real_x % self.grid.len();
            real_y = real_y % self.grid.len();
        }
        (real_x, real_y)
    }

    pub fn get_safe(&self, (x, y): (isize, isize)) -> Option<&T> {
        let (rx, ry) = self.transform((x, y));
        if self.wrap || (rx < self.grid.len() && ry < self.grid.len()) {
            Some(self.get((rx, ry)))
        } else {
            None
        }
    }
    pub fn get(&self, (x, y): (usize, usize)) -> &T {
        &self.grid[x][y]
    }

    pub fn set(&mut self, (x, y): (isize, isize), v: T) {
        let (rx, ry) = self.transform((x, y));
        self.grid[rx][ry] = v;
    }
}
impl<T: Default + Clone> OffsetGrid<T> {
    pub fn new(size: usize, offset: (isize, isize), wrap: bool) -> OffsetGrid<T> {
        OffsetGrid { grid: vec![vec![T::default(); size]; size], offset: offset, wrap: wrap }
    }
}

pub fn parse(input: &str, part2: bool) -> Input {
    let rock_coords = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cs| {
                    cs.split_once(",").map(|(x, y)| (y.parse::<isize>().unwrap(), x.parse::<isize>().unwrap())).unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut min_x = 0;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for rock in &rock_coords {
        for (x, y) in rock {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            if part2 {
                max_x = max_x.max(*x + 2);
            }
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
            if part2 {
                min_y = min_y.min(300);
                max_y = max_y.max(700);
            }
        }
    }
    let max_range = ((max_x - min_x).abs()).max((max_y - min_y).abs()) as usize;

    let mut og =
        OffsetGrid::new_with_default(max_range + 1, (-1 * (min_x as isize), -1 * (min_y as isize)), true, false);
    for rock in rock_coords {
        for pair in rock.windows(2) {
            let (c1x, c1y) = pair[0];
            let (c2x, c2y) = pair[1];
            if c1x == c2x {
                for y in c1y.min(c2y)..=c1y.max(c2y) {
                    og.set((c1x, y), false);
                }
            } else if c1y == c2y {
                for x in c1x.min(c2x)..=c1x.max(c2x) {
                    og.set((x, c1y), false);
                }
            } else {
                panic!("illegal input")
            }
        }
    }
    og.set((0, 500), false);
    if part2 {
        for y in min_y..=max_y {
            og.set((max_x, y), false);
        }
    }
    og
}
