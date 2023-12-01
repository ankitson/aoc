use itertools::Itertools;
use regex::Regex;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let input_lines: Vec<&str> = input.split('\n').collect();
        let mut current = 0;
        for i in 0..input_lines.len() {
            if (input_lines[i].len() < 2) {
                continue;
            }
            let digits = input_lines[i]
                .chars()
                .filter(|x| x.is_ascii_digit())
                .collect_vec();
            let first = digits[0].to_digit(10).unwrap() as usize;
            let last = digits[digits.len() - 1].to_digit(10).unwrap() as usize;
            current += last + (10 * first);
        }
        current
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let input_lines: Vec<&str> = input.split('\n').collect();
        let mut current = 0;
        let digit_regex =
            Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let rev_regex =
            Regex::new(r"1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        for i in 0..input_lines.len() {
            if input_lines[i].len() < 2 {
                continue;
            }
            let line = input_lines[i];

            let match_first = digit_regex.find(line).unwrap();
            let small = match match_first.as_str() {
                "1" | "one" => 1,
                "2" | "two" => 2,
                "3" | "three" => 3,
                "4" | "four" => 4,
                "5" | "five" => 5,
                "6" | "six" => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine" => 9,
                _ => panic!("Unexpected string"),
            };

            let rev_str = line.chars().rev().collect::<String>();
            let match_last = rev_regex.find(&rev_str).unwrap();
            let big = match match_last.as_str() {
                "1" | "eno" => 1,
                "2" | "owt" => 2,
                "3" | "eerht" => 3,
                "4" | "ruof" => 4,
                "5" | "evif" => 5,
                "6" | "xis" => 6,
                "7" | "neves" => 7,
                "8" | "thgie" => 8,
                "9" | "enin" => 9,
                _ => panic!("Unexpected string"),
            };
            current += big + (10 * small);
        }
        current
    }
}
