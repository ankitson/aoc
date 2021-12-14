mod soln1;

#[cfg(feature = "heapprofile")]
use dhat::{Dhat, DhatAlloc};

#[cfg_attr(feature = "heapprofile", global_allocator)]
#[cfg(feature = "heapprofile")]
static ALLOCATOR: DhatAlloc = DhatAlloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _dhat = Dhat::start_heap_profiling();
    let contents: &str = include_str!("../inputs/day5.txt");
    let part1 = soln1::Soln1::part1(contents, 1000);
    println!("Part 1: {:?}", part1);
    let part2 = soln1::Soln1::part2(contents, 1000);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample5.txt");
        let soln = soln1::Soln1::part1(sample, 10);
        assert_eq!(soln, 5);
    }
}
