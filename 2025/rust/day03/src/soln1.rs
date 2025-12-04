use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<u8>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}

fn best_digits(bank: &[u8], num_digits: usize) -> usize {
    let mut digits = Vec::new();
    let mut range_start = 0;
    let mut range_end_incl = bank.len() - num_digits;

    while digits.len() < num_digits {
        let slice = &bank[range_start..=range_end_incl];
        let best_digit = *slice.iter().max().unwrap();
        let best_digit_idx =
            bank[range_start..=range_end_incl].iter().position(|&d| d == best_digit).unwrap() + range_start;

        digits.push(best_digit);
        range_start = best_digit_idx + 1;
        range_end_incl += 1;
    }

    digits.iter().fold(0usize, |acc, &d| acc * 10 + d as usize)
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut output_p1 = 0;

    for bank in &input {
        output_p1 += best_digits(bank, 2);
    }

    output_p1
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut output_p2 = 0;

    for bank in &input {
        output_p2 += best_digits(bank, 12);
    }

    output_p2
}
