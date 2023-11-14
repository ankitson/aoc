mod shared;
mod soln1;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 07!");
    let input: &str = include_str!("../../inputs/sample07.txt");

    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 95437);

    let input: &str = include_str!("../../inputs/day07.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day07 = {:?}", part1);
    assert_eq!(part1, 1642503);

    let input: &str = include_str!("../../inputs/sample07.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample07 = {:?}", part2);
    assert_eq!(part2, 24933642);

    let input: &str = include_str!("../../inputs/day07.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day07 = {:?}", part2);
    assert_eq!(part2, 6999588);
}
