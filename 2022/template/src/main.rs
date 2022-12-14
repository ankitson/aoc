mod shared;
mod soln1;

#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
use dhat::{Dhat, DhatAlloc};

#[cfg_attr(feature = "heapprofile", global_allocator)]
#[cfg(feature = "heapprofile")]
static ALLOCATOR: DhatAlloc = DhatAlloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _dhat = Dhat::start_heap_profiling();

    println!("Hello Day 12!");
    let input: &str = include_str!("../inputs/sample12.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../inputs/day12.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day12 = {:?}", part1);

    let input: &str = include_str!("../inputs/sample12.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample12 = \n{}", part2);

    let input: &str = include_str!("../inputs/day12.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day12 = \n{}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        println!("Testing part1...");
        assert!(1 == 1)
    }
}
