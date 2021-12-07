mod soln;

use dhat::{Dhat, DhatAlloc};

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

pub fn main() {
    let _dhat = Dhat::start_heap_profiling();
    let contents: &str = include_str!("../inputs/day5.txt");
    let part1 = soln::Soln1::part1(contents, 1000);
    println!("Part 1: {:?}", part1);
    let part2 = soln::Soln1::part2(contents, 1000);
    println!("Part 2: {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample5.txt");
        let soln = soln::Soln1::part1(sample, 10);
        assert_eq!(soln, 5);
    }
}
