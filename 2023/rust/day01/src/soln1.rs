use itertools::Itertools;
use memchr::memchr3;
use memchr::memrchr3;
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
            let digits = input_lines[i].chars().filter(|x| x.is_ascii_digit()).collect_vec();
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
        let digit_regex = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let rev_regex = Regex::new(r"1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
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

    pub fn part1_memchr(raw_input: &str) -> Output {
        let mut input = parse(raw_input);
        Self::part1_memchr_core(&mut input)
    }

    pub fn part1_memchr_core(input: &mut Input) -> Output {
        let input_lines: Vec<&str> = input.split('\n').collect();
        let mut current = 0;
        for i in 0..input_lines.len() {
            let line: &str = input_lines[i];
            let line_bytes = line.as_bytes();
            if line.len() < 2 {
                continue;
            }

            let m1 = memchr3(b'1', b'2', b'3', line_bytes);
            let m2 = memchr3(b'4', b'5', b'6', line_bytes);
            let m3 = memchr3(b'7', b'8', b'9', line_bytes);

            let mr1 = memrchr3(b'1', b'2', b'3', line_bytes);
            let mr2 = memrchr3(b'4', b'5', b'6', line_bytes);
            let mr3 = memrchr3(b'7', b'8', b'9', line_bytes);

            let min_val = [m1, m2, m3].iter().filter_map(|&x| x).min().unwrap();
            let max_val = [mr1, mr2, mr3].iter().filter_map(|&x| x).max().unwrap();

            let first = (line_bytes[min_val] - b'0') as usize;
            let last = (line_bytes[max_val] - b'0') as usize;
            let incr = last + (10 * first);
            current += incr;
        }
        current
    }
}
