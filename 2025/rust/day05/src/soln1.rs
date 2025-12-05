use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<(usize, usize)>, Vec<usize>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let mut input_lines = input.lines();

    let mut ranges = Vec::new();
    for line in input_lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let (start, end) = line.split('-').collect_tuple().expect("range should contain exactly one dash");
        ranges.push((
            start.parse::<usize>().expect("range start is not a number"),
            end.parse::<usize>().expect("range end is not a number"),
        ));
    }

    // Remaining non-empty lines are the available ids.
    let available_ids = input_lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<usize>().expect("id is not a number"))
        .collect_vec();

    (ranges, available_ids)
}

pub fn part1(raw_input: &str) -> Output {
    let (ranges, available_ids) = parse(raw_input);
    let mut count = 0;
    'avail: for available_id in available_ids {
        for &(rlo, rhi) in &ranges {
            if available_id >= rlo && available_id <= rhi {
                count += 1;
                continue 'avail;
            }
        }
    }
    count
}

pub fn part2(raw_input: &str) -> Output {
    part2_merge_ranges(raw_input)
}

pub fn part2_merge_ranges(raw_input: &str) -> Output {
    let (mut ranges, _) = parse(raw_input);
    ranges.sort();
    let mut curr_range = ranges[0];
    let mut total_size = 0;
    for &(rlo, rhi) in &ranges[1..] {
        if curr_range.1 >= rlo - 1 {
            curr_range.1 = rhi.max(curr_range.1)
        } else {
            total_size += curr_range.1 - curr_range.0 + 1;
            curr_range = (rlo, rhi)
        }
    }
    total_size += curr_range.1 - curr_range.0 + 1;
    total_size
}

pub fn part2_naive(raw_input: &str) -> Output {
    let (ranges, _) = parse(raw_input);
    let mut rcount = 0;
    let mut avails = HashSet::new();
    for &(rlo, rhi) in &ranges {
        for rid in rlo..=rhi {
            avails.insert(rid);
        }
        rcount += 1;
        // println!("finished {rcount:?} ranges")
    }
    avails.len()
}
