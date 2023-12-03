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

    println!("Hello Day 03!");
    let input: &str = include_str!("../../inputs/sample03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 4361);

    let input: &str = include_str!("../../inputs/sample03.txt");
    let part1 = soln1::Soln1::part1_fast(input);
    println!("part1_fast/sample1 = {:?}", part1);
    assert_eq!(part1, 4361);

    let input: &str = include_str!("../../inputs/day03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day03 = {:?}", part1);
    assert_eq!(part1, 535078);

    let input: &str = include_str!("../../inputs/day03.txt");
    let part1 = soln1::Soln1::part1_fast(input);
    println!("part1_fast/day03 = {:?}", part1);
    assert_eq!(part1, 535078);

    let input: &str = include_str!("../../inputs/sample03.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample03 = {:?}", part2);
    assert_eq!(part2, 467835);

    let part2 = soln1::Soln1::part2_fast(input);
    println!("part2_fast/sample03 = {:?}", part2);
    assert_eq!(part2, 467835);

    let input: &str = include_str!("../../inputs/day03.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day03 = {:?}", part2);
    assert_eq!(part2, 75312571);

    let part2 = soln1::Soln1::part2_fast(input);
    println!("part2_fast/day03 = {:?}", part2);
    assert_eq!(part2, 75312571);
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
