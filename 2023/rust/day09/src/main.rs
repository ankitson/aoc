#![feature(iter_map_windows)]

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

    println!("Hello Day 09!");
    let input: &str = include_str!("../../inputs/sample09.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 114);

    let input: &str = include_str!("../../inputs/day09.txt");
    let part1 = soln1::part1(input);
    println!("part1/day09 = {:?}", part1);
    assert_eq!(part1, 1916822650);

    let input: &str = include_str!("../../inputs/day09.txt");
    let part1 = soln1::part1_noalloc(input);
    println!("part1_noalloc/day09 = {:?}", part1);
    assert_eq!(part1, 1916822650);

    let input: &str = include_str!("../../inputs/sample09.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample09 = {:?}", part2);
    assert_eq!(part2, 2);

    let input: &str = include_str!("../../inputs/day09.txt");
    let part2 = soln1::part2(input);
    println!("part2/day09 = {:?}", part2);
    assert_eq!(part2, 966);

    let part2 = soln1::part2_noalloc(input);
    println!("part2_noalloc/day09 = {:?}", part2);
    assert_eq!(part2, 966);
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
