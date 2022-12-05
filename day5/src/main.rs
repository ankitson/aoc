mod soln1;
mod shared;

#[cfg(feature = "heapprofile")]
use dhat::{Dhat, DhatAlloc};

#[cfg_attr(feature = "heapprofile", global_allocator)]
#[cfg(feature = "heapprofile")]
static ALLOCATOR: DhatAlloc = DhatAlloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _dhat = Dhat::start_heap_profiling();

    println!("Hello Day 5!");
    let input: &str = include_str!("../inputs/sample5.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Sample = {:?}", part1); //2

    let input: &str = include_str!("../inputs/day5.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Input 1 = {:?}", part1); //580

    let input: &str = include_str!("../inputs/day5.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("Part 2 / Input 1 = {:?}", part2); //895
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample5.txt");
        let soln = soln1::Soln1::part1(sample);
        assert_eq!(soln, 5);
    }
}
