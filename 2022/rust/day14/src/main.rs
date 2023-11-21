mod shared;
mod soln1;

#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 14!");
    let input: &str = include_str!("../../inputs/sample14.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 24);

    let input: &str = include_str!("../../inputs/day14.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day14 = {:?}", part1);
    assert_eq!(part1, 737);

    let input: &str = include_str!("../../inputs/sample14.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample14 = \n{}", part2);

    let input: &str = include_str!("../../inputs/day14.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day14 = \n{}", part2);
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
