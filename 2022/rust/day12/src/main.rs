mod shared;
mod soln1;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 12!");
    let input: &str = include_str!("../../inputs/sample12.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 31);

    let input: &str = include_str!("../../inputs/day12.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day12 = {:?}", part1);
    assert_eq!(part1, 425);

    let input: &str = include_str!("../../inputs/sample12.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample12 = {}", part2);
    assert_eq!(part2, 29);

    let input: &str = include_str!("../../inputs/day12.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day12 = {}", part2);
    assert_eq!(part2, 418);
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
