#![allow(unused_variables)]
#![allow(unused_mut)]
#![feature(int_roundings)]

mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 20!");
    let contents: &str = include_str!("../inputs/day20.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 =  {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2 =  {:?}", part2);
}
