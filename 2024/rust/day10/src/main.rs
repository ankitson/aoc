mod shared;
mod soln1;

#[allow(unused_imports)]
#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Hello Day 10!");
    let input: &str = include_str!("../../inputs/sample10.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10_2.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample2 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10_3.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample3 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10_4.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample4 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10_5.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample5 = {:?}", part1);

    let input: &str = include_str!("../../inputs/day10.txt");
    let part1 = soln1::part1(input);
    println!("part1/day10 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample10.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample10 = {:?}", part2);

    let input: &str = include_str!("../../inputs/sample10_6.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample10_6 = {:?}", part2);

    let input: &str = include_str!("../../inputs/sample10_7.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample10_7 = {:?}", part2);

    let input: &str = include_str!("../../inputs/sample10_8.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample10_8 = {:?}", part2);

    let input: &str = include_str!("../../inputs/sample10_5.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample10_5 = {:?}", part2);

    let input: &str = include_str!("../../inputs/day10.txt");
    let part2 = soln1::part2(input);
    println!("part2/day10 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::soln1;
    use crate::soln1::{part1, part2};

    #[test]
    fn test_part1() {
        println!("Testing part1...");
        let cases = vec![("1", 1)];
        for (inp, expected) in cases {
            assert_eq!(part1(inp), expected)
        }
    }

    #[test]
    fn test_part2() {
        println!("Testing part1...");
        let cases = vec![("1", 2)];
        for (inp, expected) in cases {
            assert_eq!(part2(inp), expected)
        }
    }
}
