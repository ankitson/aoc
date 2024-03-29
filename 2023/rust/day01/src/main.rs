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

    println!("Hello Day 01!");
    let input: &str = include_str!("../../inputs/sample01.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 142);

    let input: &str = include_str!("../../inputs/day01.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day01 = {:?}", part1);
    assert_eq!(part1, 55447);

    let input: &str = include_str!("../../inputs/sample01.txt");
    let part1 = soln1::Soln1::part1_memchr(input);
    println!("part1_memchr/sample01 = {:?}", part1);
    assert_eq!(part1, 142);

    let input: &str = include_str!("../../inputs/day01.txt");
    let part1 = soln1::Soln1::part1_memchr(input);
    println!("part1_memchr/day01 = {:?}", part1);
    assert_eq!(part1, 55447);

    let input: &str = include_str!("../../inputs/sample01_2.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample01_2 = {:?}", part2);
    assert_eq!(part2, 281);

    let input: &str = include_str!("../../inputs/day01.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day01 = {:?}", part2);
    assert_eq!(part2, 54706);
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
