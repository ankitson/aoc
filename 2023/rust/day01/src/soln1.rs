use itertools::Itertools;

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
        for i in 0..input_lines.len() {
            if (input_lines[i].len() < 2) {
                continue;
            }
            let line = input_lines[i];
            let matches1 = line.match_indices("one").collect_vec();
            let matches2 = line.match_indices("two").collect_vec();
            let matches3 = line.match_indices("three").collect_vec();
            let matches4 = line.match_indices("four").collect_vec();
            let matches5 = line.match_indices("five").collect_vec();
            let matches6 = line.match_indices("six").collect_vec();
            let matches7 = line.match_indices("seven").collect_vec();
            let matches8 = line.match_indices("eight").collect_vec();
            let matches9 = line.match_indices("nine").collect_vec();
            let mut matches = [
                matches1, matches2, matches3, matches4, matches5, matches6, matches7, matches8,
                matches9,
            ]
            .concat();

            let mut digit_indices: Vec<usize> = line
                .char_indices()
                .filter_map(|(i, c)| if c.is_ascii_digit() { Some(i) } else { None })
                .collect();
            // let match_indices: Vec<usize> = matches.map(|(index, _)| index).collect();

            matches.sort();
            digit_indices.sort();

            println!("LinEL {:?}", line);
            println!("matches: {:?}", matches);
            println!("digit indices: {:?}", digit_indices);
            let mut small = 0;
            if matches.len() == 0 || (digit_indices.len() > 0 && digit_indices[0] < matches[0].0) {
                println!(
                    "small set to {:?}",
                    line.chars().nth(digit_indices[0]).unwrap()
                );
                small = line
                    .chars()
                    .nth(digit_indices[0])
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
            } else {
                small = match matches[0].1 {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => panic!("Unexpected string"),
                };
                println!("small set to match from words {:?}", small);
            }

            let last_digit_index = {
                if digit_indices.len() > 0 {
                    digit_indices[digit_indices.len() - 1]
                } else {
                    0
                }
            };
            let last_match_indx = {
                if matches.len() > 0 {
                    matches[matches.len() - 1].0
                } else {
                    0
                }
            };
            let mut big = 0;
            if matches.len() == 0 || last_digit_index > last_match_indx {
                big = line
                    .chars()
                    .nth(last_digit_index)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
                println!("big set to match from digit indices {:?}", big);
            } else {
                big = match matches[matches.len() - 1].1 {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    _ => panic!("Unexpected string"),
                };
                println!("big set to match from words {:?}", big);
            }
            println!("digits are {} {}", { small }, { big });
            current += big + (10 * small);
        }
        current
    }
}
