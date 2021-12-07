mod soln;

pub fn main() {
    let contents: &str = include_str!("../inputs/day7.txt");
    let part1 = soln::Soln1::part1(contents);
    println!("{:?}", part1);
    let part2 = soln::Soln1::part2(contents);
    println!("{:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample7.txt");
        let soln = soln::Soln1::part1(sample);
        assert_eq!(soln, 37);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample7.txt");
        let soln = soln::Soln1::part2(sample);
        assert_eq!(soln, 168);
    }
}
