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

    println!("Hello Day 06!");
    let input: &str = include_str!("../../inputs/sample06.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 288);

    let input: &str = include_str!("../../inputs/day06.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day06 = {:?}", part1);
    assert_eq!(part1, 1731600);

    let input: &str = include_str!("../../inputs/sample06.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample06 = {:?}", part2);
    assert_eq!(part2, 71503);

    let input: &str = include_str!("../../inputs/sample06.txt");
    let part2 = soln1::Soln1::part2_other(input);
    println!("part2_other/sample06 = {:?}", part2);
    assert_eq!(part2, 71503);

    let input: &str = include_str!("../../inputs/day06.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day06 = {:?}", part2);
    assert_eq!(part2, 40087680);

    let input: &str = include_str!("../../inputs/day06.txt");
    let part2 = soln1::Soln1::part2_other(input);
    println!("part2_other/day06 = {:?}", part2);
    assert_eq!(part2, 40087680);
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
