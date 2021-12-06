mod soln;

pub fn main() {
    let contents: &str = include_str!("../inputs/day6.txt");
    // let part1 = soln::Soln1::part1(contents, 1000);
    // println!("{:?}", part1);
    // let part2 = soln::Soln1::part2(contents, 1000);
    // println!("{:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample6.txt");
        let soln = soln::Soln1::part1(sample, 10);
        assert_eq!(soln, 5);
    }
}