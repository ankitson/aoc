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

    println!("Hello Day 09!");
    let input: &str = include_str!("../../inputs/sample09.txt");
    let part1 = soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);
    assert_eq!(part1, 1928);

    let input: &str = include_str!("../../inputs/day09.txt");
    let part1 = soln1::part1(input);
    println!("part1/day09 = {:?}", part1);
    assert_eq!(part1, 6288599492129);

    let input: &str = include_str!("../../inputs/sample09.txt");
    let part2 = soln1::part2(input);
    println!("part2/sample09 = {:?}", part2);
    assert_eq!(part2, 2858);

    let input: &str = include_str!("../../inputs/day09.txt");
    let part2 = soln1::part2(input);
    println!("part2/day09 = {:?}", part2);
    assert_eq!(part2, 6321896265143);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::soln1;
    use crate::soln1::part2;

    #[test]
    fn test_part2() {
        println!("Testing part2...");
        let cases = vec![
            ("714892711", 813),
            ("12101", 4),
            ("12345", 132),
            ("233313312141413140211", 2910),
            ("1313165", 169),
            ("80893804751608292", 1715),
        ];
        for (input, expected) in cases {
            assert_eq!(part2(input), expected)
        }
    }
}
