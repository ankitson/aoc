mod shared;
mod soln1;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 02!");
    let input: &str = include_str!("../../inputs/sample02.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 8);

    let part1 = soln1::Soln1::part1_parsing(input);
    println!("part1_parsing/sample1 = {:?}", part1);
    assert_eq!(part1, 8);

    let input: &str = include_str!("../../inputs/day02.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day02 = {:?}", part1);
    assert_eq!(part1, 2256);

    let part1 = soln1::Soln1::part1_parsing(input);
    println!("part1_parsing/day02 = {:?}", part1);
    assert_eq!(part1, 2256);

    let input: &str = include_str!("../../inputs/sample02.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample02 = {:?}", part2);
    assert_eq!(part2, 2286);

    let part2 = soln1::Soln1::part2_parsing(input);
    println!("part2_parsing/sample02 = {:?}", part2);
    assert_eq!(part2, 2286);

    let input: &str = include_str!("../../inputs/day02.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day02 = {:?}", part2);
    assert_eq!(part2, 74229);

    let part2 = soln1::Soln1::part2_parsing(input);
    println!("part2_parsing/day02 = {:?}", part2);
    assert_eq!(part2, 74229);
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
