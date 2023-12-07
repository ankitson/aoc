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
        let lines = input.lines().filter(|x| x.len() > 1).collect_vec();
        let seeds = lines[0].split_once(": ").unwrap().1.split(' ').map(|x| x.parse::<usize>().unwrap()).collect_vec();

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

        let mut min_idx2: usize = usize::MAX;
        let mins = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0).min().unwrap() as i64).collect_vec();
        let maxs = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0 + x.2).max().unwrap() as i64).collect_vec();

        let min_idx = seeds
            .iter() //S seeds
            .fold(usize::MAX, |current_min, seed| {
                let mut curr = *seed as i64;
                for depth in 0..depth_maps.len() {
                    //D depth = constant 8
                    let mut mapped = false;
                    if curr < mins[depth] || curr > maxs[depth] {
                        continue; //it isnt in any range so it maps to itself
                    }

                    let mut next = 0;
                    for (src, dst, rnglen) in depth_maps.get(depth).unwrap() {
                        //K maps per depth
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
                current_min.min(curr as usize)
            });
        min_idx
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

        let mut min_idx: usize = usize::MAX;
        let mins = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0).min().unwrap() as i64).collect_vec();
        let maxs = depth_maps.iter().map(|dm| dm.iter().map(|x| x.0 + x.2).max().unwrap() as i64).collect_vec();

        for i in (0..seed_ranges.len() - 1).step_by(2) {
            let start = seed_ranges[i];
            let rangelen = seed_ranges[i + 1];
            let res = (start..=start + rangelen) //S seeds
                .fold(usize::MAX, |current_min, seed| {
                    let mut curr = seed as i64;
                    for depth in 0..depth_maps.len() {
                        //D depth = constant 8
                        let mut mapped = false;
                        if curr < mins[depth] || curr > maxs[depth] {
                            continue; //it isnt in any range so it maps to itself
                        }

                        let mut next = 0;
                        for (src, dst, rnglen) in depth_maps.get(depth).unwrap() {
                            //K maps per depth
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
                    // let mut new_memo = memo.clone();
                    // new_memo.insert(seed, curr);
                    // (current_min.min(curr as usize), new_memo)
                    current_min.min(curr as usize)
                });
            // .map(|x| x.0)
            // .min()
            // .unwrap();
            min_idx = min_idx.min(res);
        }
        min_idx
    }

    pub fn part2_faster(input: &Input) -> Output {
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
                current_map.sort_unstable_by_key(|(s, d, r)| d);
                depth_maps.push(current_map);
                current_map = vec![];
            }
        }
        current_map.sort_unstable_by_key(|(s, d, r)| d);
        depth_maps.push(current_map);

        let mut dmi = depth_maps.len() - 1;
        let mut dm = depth_maps[dmi];
        // let next
        for (src, dst, rnglen) in dm {}
    }
}
