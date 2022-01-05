#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(int_roundings)]

mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 19!");
    let contents: &str = include_str!("../inputs/day19.txt");
    let part1 = soln1::part1_attempt2(contents).len();
    println!("Part 1 =  {:?}", part1);
    let part2 = soln1::part2(contents);
    println!("Part 2 =  {:?}", part2);
}
