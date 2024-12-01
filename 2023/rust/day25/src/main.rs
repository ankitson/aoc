mod shared;
mod soln1;

#[allow(unused_imports)]
#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::Write;
#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 25!");
    let input: &str = include_str!("../../inputs/sample25.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day25.txt");
    let part1 = soln1::part1(input);
    println!("part1/day25 = {}", part1);
    // let mut file = File::create("output_part1.txt").expect("Unable to create file");
    // write!(file, "{}", part1).expect("Unable to write data");
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
