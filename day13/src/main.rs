mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 13!");
    let contents: &str = include_str!("../inputs/day13.txt");
    let part1 = soln1::Soln1::part1(contents, 2000);
    println!("Part 1 = {:?}", part1);
    let part2 = soln1::Soln1::part2(contents, 2000);
    println!("Part 2 =\n{}", part2);
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;
    use crate::soln1::Soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let part1 = Soln1::part1(contents, 20);
        println!("{}", part1);
        assert_eq!(part1, 17);
        let part2 = Soln1::part2(contents, 20);
        println!("{}", part2);
        // assert_eq!(part2, Some(195));
    }
}
