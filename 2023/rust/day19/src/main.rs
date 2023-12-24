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
    let sample: &str = include_str!("../../inputs/sample19.txt");
    let input: &str = include_str!("../../inputs/day19.txt");

    println!("Hello Day 19!");
    let part1 = soln1::part1(sample);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 19114);

    let part1 = soln1::part1(input);
    println!("part1/day19 = {:?}", part1);
    assert_eq!(part1, 331208);

    let part2 = soln1::part2(sample);
    println!("part2/sample19 = {:?}", part2);
    assert_eq!(part2, 167409079868000);

    let part2 = soln1::part2(input);
    println!("part2/day19 = {:?}", part2);
    assert_eq!(part2, 121464316215623)
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
