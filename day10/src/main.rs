#![feature(drain_filter)]
mod soln1;

pub fn main() {
    println!("Hello Day 10!");
    let contents: &str = include_str!("../inputs/day10.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 = {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let part1 = soln1::Soln1::part1(contents);
        assert_eq!(part1, 26397);
        let part2 = soln1::Soln1::part2(contents);
        assert_eq!(part2, 288957);
    }
}
