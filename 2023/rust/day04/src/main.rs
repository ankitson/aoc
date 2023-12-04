mod shared;
mod soln1;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 04!");
    let input: &str = include_str!("../../inputs/sample04.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 13);

    let input: &str = include_str!("../../inputs/day04.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day04 = {:?}", part1);
    assert_eq!(part1, 19135);

    let input: &str = include_str!("../../inputs/sample04.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample04 = {:?}", part2);
    assert_eq!(part2, 30);

    let input: &str = include_str!("../../inputs/day04.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day04 = {:?}", part2);
    assert_eq!(part2, 5704953);
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
