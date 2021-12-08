#![feature(drain_filter)]
mod soln;

pub fn main() {
    println!("Hello Day 8!");
    let contents: &str = include_str!("../inputs/day8.txt");
    let part1 = soln::Soln1::part1(contents);
    println!("Part 1: {:?}", part1);
    // let part2 = soln::Soln1::part2(contents);
    // println!("Part 2: {:?}", part2);
    // let part1 = soln::Soln1::part1_fast(contents);
    // println!("Part 1 (quickselect+median): {:?}", part1);
    // let part2 = soln::Soln1::part2_fast(contents);
    // println!("Part 2 (mean): {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    #[test]
    fn test_part1() {
        //TODO: Fix parsing
        let sample: &str = include_str!("../inputs/sample8.txt");
        let part1 = soln::Soln1::part1(sample);
        println!("Part 1: {}", part1);
        assert_eq!(part1, 26);
    }
}
