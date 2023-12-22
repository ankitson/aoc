#![feature(let_chains)]
mod shared;
mod soln1;

#[allow(unused_imports)]
#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();
    let sample: &str = include_str!("../../inputs/sample21.txt");
    let input: &str = include_str!("../../inputs/day21.txt");
    println!("Hello Day 21!");

    let part1 = soln1::part1(sample, 6);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 16);

    let part1 = soln1::part1(input, 64);
    println!("part1/day21 = {:?}", part1);
    assert_eq!(part1, 3532);

    for (nstep, exp) in [(6, 16), (10, 50), (50, 1594), (100, 6536)] {
        let part2 = soln1::part2(sample, nstep, false);
        println!("part2/sample1({nstep}) = {:?}", part2);
        assert_eq!(part2, exp);
    }

    println!("Part 2 - here are the first 200 values of the function. Use the python notebook for the rest");
    const NSTEPS_P2: usize = 26501365;
    let part2 = soln1::part2(input, 200, true);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::soln1;

    #[test]
    fn test_part1() {
        println!("Testing part1...");
        assert!(1 == 1)
    }
}
