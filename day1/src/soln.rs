use std::{fmt::Display, net::IpAddr};

pub struct Day1 {}

impl Solution for Day1 {}

// fn parse(input: &str) -> InputType {
//     input
//         .split('\n')
//         .map(|x| x.parse::<i32>().expect("Unable to parse int"))
//         .collect()
// }

// fn unparse(output: OutputType) -> impl Display {

// }

// fn count_window(input: Vec<i32>, window_size: usize) -> i32 {
//     let mut count = 0;
//     for i in 0..input.len() - window_size {
//         count += if input[i + window_size] > input[i] { 1 } else { 0 };
//     }
//     count
// }

// type InputType = Vec<i32>
// type OutputType = i32

// pub fn part1() -> String {
//     count_window(parse(), 1).to_string()
// }

// pub fn part1(raw_input: &str) -> impl Display {
//     let input = parse(raw_input)

// }
// pub fn part1_core(input: InputType) -> OutputType {
//     count_window(input, 1)
// }

// pub fn part2() -> String {
//     count_window(parse(), 3).to_string()
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// fn main() {
//     //let input = parse();
//     let contents: &str = include_str!("../day1.txt");
//     println!("{}", part1());
// }
