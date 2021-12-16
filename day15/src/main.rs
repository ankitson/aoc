mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 15!");
    let contents: &str = include_str!("../inputs/day15.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 =  {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2 =  {:?}", part2);
}

#[cfg(test)]
mod tests {
    use day15::soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = soln1::Soln1::part1_slow(sample);
        println!("Part 1 (slow) = {:?}", part1);
        assert_eq!(part1, 40);

        let part1_djikstra = soln1::Soln1::part1(sample);
        println!("Part 1 (djikstra) = {:?}", part1);
        assert_eq!(part1_djikstra, 40);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = soln1::Soln1::part2(sample);
        println!("Part 2 (djikstra) = {:?}", part2);
        assert_eq!(part2, 315);
    }
}
