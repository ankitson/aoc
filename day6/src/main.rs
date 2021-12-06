mod soln;

pub fn main() {
    let contents: &str = include_str!("../inputs/day6.txt");
    let part1 = soln::Soln1::part1(contents, 80);
    println!("{:?}", part1);
    let part2 = soln::Soln1::part2(contents, 256);
    println!("{:?}", part2);
    // let part2 = soln::Soln1::part2(contents, 1000);
    // println!("{:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample6.txt");
        let soln = soln::Soln1::part1(sample, 18);
        assert_eq!(soln, 26);
        let soln = soln::Soln1::part1(sample, 80);
        assert_eq!(soln, 5934);
        let soln = soln::Soln1::part2(sample, 18);
        assert_eq!(soln, 26);
        let soln = soln::Soln1::part2(sample, 80);
        assert_eq!(soln, 5934);
    }
}
