mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 15!");
    let contents: &str = include_str!("../inputs/day15.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 =  {:?}", part1);
    let part1_fast = soln1::Soln1::part1_fast(contents);
    println!("Part 1 (fast) =  {:?}", part1_fast);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2 =  {:?}", part2);
}

#[cfg(test)]
mod tests {
    use day15::soln1;

    use crate::shared::parse;
    use crate::soln1::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = soln1::Soln1::part1(sample);
        println!("Part 1 (sample) = {:?}", part1);
        assert_eq!(part1, 40);
        let part1_fast = soln1::Soln1::part1_fast(sample);
        println!("Part 1 (sample) = {:?}", part1);
        assert_eq!(part1_fast, 40);

        let sample2: &str = include_str!("../inputs/sample2.txt");
        let part1 = soln1::Soln1::part1(sample2);
        println!("Part 1 (sample2) = {:?}", part1);
        let part1_fast = soln1::Soln1::part1_fast(sample2);
        println!("Part 1 fast (sample2) = {:?}", part1_fast);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = soln1::Soln1::part2(sample);
        println!("Part 2 (sample) = {:?}", part2);
        assert_eq!(part2, 315);
        // let sample2: &str = include_str!("../inputs/sample2.txt");
        // let sample3: &str = include_str!("../inputs/sample3.txt");
        // test_part2_with_sample(sample1, 40);
        // test_part2_with_sample(sample2, 103);
        // test_part2_with_sample(sample3, 3509);
    }
}
