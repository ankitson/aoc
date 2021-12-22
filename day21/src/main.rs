#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]

use std::collections::HashMap;

mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 21!");
    let contents: &str = include_str!("../inputs/day21.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 =  {:?}", part1);

    let sample: &str = include_str!("../inputs/sample.txt");
    let cache = HashMap::new();
    let mut soln1 = soln1::Soln1::new(cache);
    for target in 2..3 {
        println!("Part 2 (sample) (target = {}) = {:?}", target, soln1.part2(sample, target));
        soln1.cache.clear();
    }

    // println!("{}", soln1::Soln1::part22(contents));

    // println!("Part 2 (sample) = {:?}", soln1::Soln1::part2(sample));

    // let part2 = soln1::Soln1::part2(contents);
    // println!("Part 2 =  {:?}", part2);
}
