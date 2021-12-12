#![feature(array_zip)]
mod soln1;

pub fn main() {
    println!("Hello Day 12!");
    // let contents: &str = include_str!("../inputs/day12.txt");
    // let part1 = soln1::Soln1::part1(contents, 100);
    // println!("Part 1 = {:?}", part1);
    // let part2 = soln1::Soln1::part2(contents);
    // println!("Part 2 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        // let part1 = soln1::Soln1::part1(contents, 100);
        // assert_eq!(part1, 1656);
        // let part2 = soln1::Soln1::part2(contents);
        // println!("{:?}", part2);
        // assert_eq!(part2, Some(195));
    }
}
