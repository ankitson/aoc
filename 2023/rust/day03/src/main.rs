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

    println!("Hello Day 03!");
    let input: &str = include_str!("../../inputs/sample03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day03 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample03.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample03 = {:?}", part2);

    let input: &str = include_str!("../../inputs/day03.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day03 = {:?}", part2);
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
