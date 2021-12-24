#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]

mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 23!");
    let contents: &str = include_str!("../inputs/day23.txt");
    let part1 = soln1::Soln1::<3>::part1(contents);
    println!("Part 1 =  {:?}", part1);
    // let part2 = soln1::Soln1::part2(contents);
    // println!("Part 2 =  {:?}", part2);
}
