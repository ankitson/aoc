use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<String>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let parts =
        input.lines().map(|line| line.split_ascii_whitespace().map(|x| x.to_string()).collect_vec()).collect_vec();
    parts
}

pub fn part1(raw_input: &str) -> Output {
    let parts = parse(raw_input);
    let (nums, ops_last) = parts.split_at(parts.len() - 1);
    let ops = ops_last[0].clone();
    let parsed_nums =
        nums.into_iter().map(|row| row.into_iter().map(|x| x.parse::<usize>().unwrap()).collect_vec()).collect_vec();
    let mut total = 0;
    for elt_idx in 0..parsed_nums[0].len() {
        let opchar = &ops[elt_idx].chars().nth(0).expect("illegal op");
        let mut col_val = 0;
        if *opchar == '*' || *opchar == '/' {
            col_val = 1;
        }
        for row in &parsed_nums {
            let elt = row[elt_idx];
            match opchar {
                '+' => col_val += elt,
                '-' => col_val -= elt,
                '*' => col_val *= elt,
                '/' => col_val /= elt,
                _ => panic!("illegal op"),
            }
        }
        println!("col {elt_idx:?} val = {col_val:?}");
        total += col_val
    }
    total
}

pub fn part2(raw_input: &str) -> Output {
    let parts = parse(raw_input);
    let (nums, ops_last) = parts.split_at(parts.len() - 1);
    let ops = ops_last[0].clone();
    for col_idx in 0..100 {
        for row in nums {}
    }
    todo!()
}
