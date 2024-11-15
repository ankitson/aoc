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
    let sample: &str = include_str!("../../inputs/sample22.txt");
    let input: &str = include_str!("../../inputs/day22.txt");

    println!("Hello Day 22!");
    let part1 = soln1::part1(sample);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 5);

    let part1 = soln1::part1(input);
    println!("part1/day22 = {:?}", part1);
    assert_eq!(part1, 409);

    let part2 = soln1::part2(sample);
    println!("part2/sample22 = {:?}", part2);
    assert_eq!(part2, 7);

    let part2 = soln1::part2(input);
    println!("part2/day22 = {:?}", part2);
    assert_eq!(part2, 61097);
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
