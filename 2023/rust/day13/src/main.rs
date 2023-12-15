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

    println!("Hello Day 13!");
    let input: &str = include_str!("../../inputs/sample13.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day13.txt");
    let part1 = soln1::part1(input);
    println!("part1/day13 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample13.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample13 = {:?}", part2);

    let input: &str = include_str!("../../inputs/day13.txt");
    let part2 = soln1::part2(input);
    println!("part2/day13 = {:?}", part2);
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
