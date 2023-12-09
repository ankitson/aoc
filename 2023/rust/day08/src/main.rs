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

    println!("Hello Day 08!");
    let input: &str = include_str!("../../inputs/sample08.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 2);

    let input: &str = include_str!("../../inputs/day08.txt");
    let part1 = soln1::part1(input);
    println!("part1/day08 = {:?}", part1);
    assert_eq!(part1, 16409);

    let input: &str = include_str!("../../inputs/sample08_2.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample08 = {:?}", part2);
    assert_eq!(part2, 6);

    let input: &str = include_str!("../../inputs/day08.txt");
    let part2 = soln1::part2(input);
    println!("part2/day08 = {:?}", part2);
    assert_eq!(part2, 11795205644011);
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
