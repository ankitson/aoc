mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 14!");
    let contents: &str = include_str!("../inputs/day14.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 = {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2 =\n{}", part2);
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;
    use crate::soln1::Soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let parsed = parse(contents);
        println!("{:?}", parsed);
        let part1 = Soln1::part1(contents);
        println!("{}", part1);
        assert_eq!(part1, 1588);
        let part2 = Soln1::part2(contents);
        println!("{}", part2);
        assert_eq!(part2, 2188189693529);
    }
}
