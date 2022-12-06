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

    println!("Hello Day 6!");
    let input: &str = include_str!("../inputs/sample6.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample6 = {:?}", part1);

    let input: &str = include_str!("../inputs/day6.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day6 = {:?}", part1);

    let input: &str = include_str!("../inputs/sample6.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample6 = {:?}", part2);

    let input: &str = include_str!("../inputs/day6.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day6 = {:?}", part2);

    let input: &str = include_str!("../inputs/sample6.txt");
    let part2 = soln1::Soln1::part2_windows(input);
    println!("part2_windows/sample6 = {:?}", part2);

    let input: &str = include_str!("../inputs/day6.txt");
    let part2 = soln1::Soln1::part2_windows(input);
    println!("part2_windows/day6 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        assert_eq!(soln1::Soln1::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(soln1::Soln1::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(soln1::Soln1::part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(soln1::Soln1::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(soln1::Soln1::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(soln1::Soln1::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(soln1::Soln1::part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(soln1::Soln1::part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(soln1::Soln1::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(soln1::Soln1::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

        //BUGGY
        // assert_eq!(soln1::Soln1::part2_set("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        // assert_eq!(soln1::Soln1::part2_set("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        // assert_eq!(soln1::Soln1::part2_set("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        // assert_eq!(soln1::Soln1::part2_set("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        // assert_eq!(soln1::Soln1::part2_set("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

        assert_eq!(soln1::Soln1::part2_windows("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(soln1::Soln1::part2_windows("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(soln1::Soln1::part2_windows("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(soln1::Soln1::part2_windows("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(soln1::Soln1::part2_windows("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
