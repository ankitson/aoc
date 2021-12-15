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
    use crate::shared::parse;
    use crate::soln1::Soln1;

    #[test]
    fn test_part1() {
        let sample1: &str = include_str!("../inputs/sample.txt");
        // let sample2: &str = include_str!("../inputs/sample2.txt");
        // let sample3: &str = include_str!("../inputs/sample3.txt");
        // test_part1_with_sample(sample1, 10);
        // test_part1_with_sample(sample2, 19);
        // test_part1_with_sample(sample3, 226);
    }

    #[test]
    fn test_part2() {
        let sample1: &str = include_str!("../inputs/sample.txt");
        // let sample2: &str = include_str!("../inputs/sample2.txt");
        // let sample3: &str = include_str!("../inputs/sample3.txt");
        // test_part2_with_sample(sample1, 36);
        // test_part2_with_sample(sample2, 103);
        // test_part2_with_sample(sample3, 3509);
    }

    fn test_part1_with_sample(sample: &str, expected: u64) {
        // let part1 = soln1::Soln1::part1(sample);
        // assert_eq!(part1, expected);
    }

    fn test_part2_with_sample(sample: &str, expected: u64) {
        // let soln = soln1::Soln1::part2(sample);
        // assert_eq!(soln, expected);
    }
}
