use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<(i64, i64)>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let line = input.lines().next().unwrap_or("");
    line.split(',')
        .map(|range_str| {
            let parts: Vec<&str> = range_str.split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect()
}

fn is_repeat(s: &str) -> bool {
    let len = s.len();
    for repeat_len in 1..=len / 2 {
        let seq = &s[0..repeat_len];
        let mut window_start = 0;
        let mut this_win = true;

        while window_start < len {
            let end = (window_start + repeat_len).min(len);
            if &s[window_start..end] != seq {
                this_win = false;
                break;
            }
            window_start += repeat_len;
        }

        if this_win {
            return true;
        }
    }
    false
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut count_p1: i64 = 0;

    for (start, end) in input {
        for i in start..=end {
            let str_i = i.to_string();
            let half_len = str_i.len() / 2;
            if str_i[0..half_len] == str_i[half_len..] {
                count_p1 += i;
            }
        }
    }

    count_p1 as usize
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut count_p2: i64 = 0;

    for (start, end) in input {
        for i in start..=end {
            let str_i = i.to_string();
            if is_repeat(&str_i) {
                count_p2 += i;
            }
        }
    }

    count_p2 as usize
}
