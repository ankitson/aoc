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

    println!("Hello Day 11!");
    let input: &str = include_str!("../../inputs/sample11.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 10605);

    let input: &str = include_str!("../../inputs/day11.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day11 = {:?}", part1);
    assert_eq!(part1, 108240);

    let input: &str = include_str!("../../inputs/sample11.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample11 = {}", part2);
    assert_eq!(part2, 2713310158);

    let input: &str = include_str!("../../inputs/day11.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day11 = {}", part2);
    assert_eq!(part2, 25712998901);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        println!("Testing part1...");
    }
}
