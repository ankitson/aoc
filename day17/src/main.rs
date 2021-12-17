mod shared;
mod soln1;

pub fn main() {
    println!("Hello Day 17!");
    let contents: &str = include_str!("../inputs/day17.txt");
    // let part1 = soln1::Soln1::part1(contents);
    // println!("Part 1 =  {:?}", part1);
    // let part2 = soln1::Soln1::part2(contents);
    // println!("Part 2 =  {:?}", part2);
}

#[cfg(test)]
mod tests {
    // use day17::{shared, soln1};

    #[test]
    fn test_part1() {
        // let sample: &str = include_str!("../inputs/sample.txt");
        // let part2 = soln1::Soln1::part2(sample);
        // println!("Part 2 (djikstra) = {:?}", part2);
        // assert_eq!(part2, 315);
    }
}
