#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]

mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 22!");
    let sample: &str = include_str!("../inputs/sample.txt");
    let sample2: &str = include_str!("../inputs/sample2.txt");
    let contents: &str = include_str!("../inputs/day22.txt");
    // run(sample2, "sample2");
    // run(sample, "sample");
    run(contents, "input");
}

fn run(input: &str, tag: &str) {
    // let part1 = soln1::Soln1::part1(input);
    // println!("Part 1 ({})=  {:?}", tag, part1);
    let part2 = soln1::Soln1::part2(input);
    println!("Part 2 ({})=  {:?}", tag, part2);
}
