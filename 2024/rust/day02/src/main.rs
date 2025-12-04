#![feature(array_windows)]
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

    println!("Hello Day 02!");
    let sample_input: &str = include_str!("../../inputs/sample02.txt");
    let part1 = soln1::part1(sample_input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 2);

    let real_input: &str = include_str!("../../inputs/day02.txt");
    let part1 = soln1::part1(real_input);
    println!("part1/day02 = {:?}", part1);
    assert_eq!(part1, 598);

    // let input: &str = include_str!("../../inputs/sample02.txt");
    let part2 = soln1::part2(sample_input);
    println!("part2/sample02 = {:?}", part2);
    assert_eq!(part2, 4);

    // let input: &str = include_str!("../../inputs/day02.txt");
    let part2 = soln1::part2(real_input);
    println!("part2/day02 = {:?}", part2);
    assert_eq!(part2, 634);
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
