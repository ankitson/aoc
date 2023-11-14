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

    println!("Hello Day 8!");
    let input: &str = include_str!("../../inputs/sample08.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample8 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day08.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day8 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample08.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample8 = {:?}", part2);

    let input: &str = include_str!("../../inputs/day08.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day8 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        // assert_eq!(soln1::Soln1::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 8);
        // assert_eq!(soln1::Soln1::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        // assert_eq!(soln1::Soln1::part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        // assert_eq!(soln1::Soln1::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        // assert_eq!(soln1::Soln1::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
