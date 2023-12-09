use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<isize>>;
pub type Output = isize;

pub fn parse(input: &str) -> Input {
    let parsed = input
        .lines()
        .map(|line| line.split_ascii_whitespace().filter_map(|n| n.parse().ok()).collect_vec())
        .collect_vec();
    parsed
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    fn predict_next(mut seq: Vec<isize>, sl: usize) -> isize {
        let mut all_0 = true;
        let mut last = seq[sl - 1];
        for i in 0..sl - 1 {
            let a = seq[i];
            let b = seq[i + 1];
            if !(a == 0 && b == 0) {
                all_0 = false;
            }
            seq[i] = b - a;
        }
        if all_0 {
            return 0;
        }
        last + predict_next(seq, sl - 1)
    }
    input
        .into_iter()
        .map(move |seq| {
            let sl = seq.len();
            predict_next(seq, sl)
        })
        .sum()
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    fn predict_next(seq: &[isize]) -> isize {
        if seq.iter().all(|x| *x == 0) {
            return 0;
        }
        let diffs = ((0 as isize)..(seq.len() as isize))
            .map_windows(|win: &[isize; 2]| seq[win[1] as usize] - seq[win[0] as usize])
            .collect_vec();
        seq.first().unwrap() - predict_next(&diffs)
    }
    input.iter().map(|seq| predict_next(seq)).sum()
}
