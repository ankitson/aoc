#![feature(array_zip)]
mod soln1;

pub fn main() {
    println!("Hello Day 10!");
    let contents: &str = include_str!("../inputs/day10.txt");
    let soln = soln1::Soln1::new();
    let part1 = soln1::Soln1::part1(&soln, contents);
    println!("Part 1 = {:?}", part1);
    let part2 = soln1::Soln1::part2(&soln, contents);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let soln = soln1::Soln1::new();
        let part1 = soln1::Soln1::part1(&soln, contents);
        assert_eq!(part1, 26397);
        let part2 = soln1::Soln1::part2(&soln, contents);
        assert_eq!(part2, 288957);
    }
}
