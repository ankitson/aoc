mod shared;
mod soln1;

#[cfg(feature = "heapprofile")]
use dhat::{Dhat, DhatAlloc};

#[cfg_attr(feature = "heapprofile", global_allocator)]
#[cfg(feature = "heapprofile")]
static ALLOCATOR: DhatAlloc = DhatAlloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _dhat = Dhat::start_heap_profiling();

    println!("Hello Day 10!");
    let input: &str = include_str!("../../inputs/sample10.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample10 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10_2.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample10_2 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day10.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day10 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample10 = \n{}", part2);

    let input: &str = include_str!("../../inputs/sample10_2.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample10_2 = \n{}", part2);

    let input: &str = include_str!("../../inputs/day10.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day10 = \n{}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {}
}
