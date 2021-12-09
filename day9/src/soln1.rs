use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> u32 {
        let heights = crate::shared::parse(input);

        let mut risk: u32 = 0;
        for i in 0..heights.len() {
            for j in 0..heights[0].len() {
                let nbr = Self::nbrs(i, j, &heights);
                let pt = heights[i][j];
                let mut lower = true;
                for (nx, ny) in nbr {
                    if (pt >= heights[nx][ny]) {
                        lower = false;
                    }
                }
                if lower {
                    risk += u32::from(pt) + 1;
                }
            }
        }
        risk
    }

    fn nbrs(x: usize, y: usize, hts: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
        let maxX = hts.len();
        let maxY = hts[0].len();
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if x != 0 {
            nbrs.push((x - 1, y));
        }
        if x + 1 < maxX {
            nbrs.push((x + 1, y));
        }
        if y != 0 {
            nbrs.push((x, y - 1));
        }
        if y + 1 < maxY {
            nbrs.push((x, y + 1));
        }

        nbrs
    }
}
