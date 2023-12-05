use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        0
        // todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let lines = input.lines().filter(|x| x.len() > 1).collect_vec();
        let seed_ranges =
            lines[0].split_once(": ").unwrap().1.split(' ').map(|x| x.parse::<usize>().unwrap()).collect_vec();

        let mut current_map = vec![];
        let mut depth_maps = vec![];
        for line in &lines[3..] {
            if line.chars().nth(0).unwrap().is_numeric() {
                let parts = line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect_vec();
                current_map.push((parts[1], parts[0], parts[2]))
            } else {
                depth_maps.push(current_map);
                current_map = vec![];
            }
        }
        depth_maps.push(current_map);

        println!("seed ranges: {seed_ranges:?}");
        println!("mapss: {depth_maps:?}");

        let mut min_idx: usize = usize::MAX;
        // let mut memo: HashMap<usize, i64> = HashMap::new();

        let mut mins = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0).min().unwrap() as i64).collect_vec();
        let mut maxs = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0 + x.2).max().unwrap() as i64).collect_vec();

        // seed_ranges.par_bridge().
        for i in (0..seed_ranges.len() - 1).step_by(2) {
            let start = seed_ranges[i];
            let rangelen = seed_ranges[i + 1];
            let res = (start..=start + rangelen)
                .into_par_iter()
                .fold_with((usize::MAX, HashMap::new()), |(current_min, memo), seed| {
                    if !memo.contains_key(&seed) {
                        let mut curr = seed as i64;
                        for depth in 0..depth_maps.len() {
                            let mut mapped = false;
                            if curr < mins[depth] || curr > maxs[depth] {
                                continue;
                            }

                            let mut next = 0;
                            for (src, dst, rnglen) in depth_maps.get(depth).unwrap() {
                                let srcoff = curr - (*src as i64);
                                if srcoff < 0 || srcoff > (*rnglen as i64) {
                                    continue;
                                } else if srcoff < (*rnglen as i64) && srcoff >= 0 {
                                    mapped = true;
                                    next = (*dst as i64) + srcoff;
                                    break;
                                }
                            }
                            if !mapped {
                                next = curr;
                            }
                            curr = next;
                        }
                        let mut new_memo = memo.clone();
                        new_memo.insert(seed, curr);
                        (current_min.min(curr as usize), new_memo)
                    } else {
                        (current_min, memo)
                    }
                })
                .map(|x| x.0)
                .min()
                .unwrap();
            min_idx = min_idx.min(res);
        }
        min_idx
    }
}
