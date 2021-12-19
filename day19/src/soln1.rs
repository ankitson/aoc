use crate::shared::parse;
pub fn part1(input: &str) -> usize {
    todo!()
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::soln1::{part1, part2};

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = part1(sample);
        println!("Part 1 (sample1) = {:?}", part1);
        // assert_eq!(part1, 0);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = part2(sample);
        println!("Part 2 (sample2) = {:?}", part2);
        // assert_eq!(part2, 0);
    }
}
