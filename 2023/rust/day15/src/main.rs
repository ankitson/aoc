use std::time::{Instant, SystemTime, UNIX_EPOCH};

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

    println!("Hello Day 15!");
    let sample: &str = include_str!("../../inputs/sample15.txt");
    let input: &str = include_str!("../../inputs/day15.txt");

    let part1 = soln1::part1(sample);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 1320);

    let start = Instant::now();
    let part1 = soln1::part1(input);
    let elapsed = start.elapsed().as_micros();
    println!("{elapsed}μs part1/day15 = {:?}", part1);
    assert_eq!(part1, 520500);

    let start = Instant::now();
    let part1 = soln1::part1_rayon(input);
    let elapsed = start.elapsed().as_micros();
    println!("{elapsed}μs: part1_rayon/day15 = {:?}", part1);
    assert_eq!(part1, 520500);

    let part2 = soln1::part2(sample);
    println!("part2/sample15 = {:?}", part2);
    assert_eq!(part2, 145);

    let part2 = soln1::part2(input);
    println!("part2/day15 = {:?}", part2);
    assert_eq!(part2, 213097);
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
