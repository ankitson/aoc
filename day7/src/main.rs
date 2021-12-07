#![feature(drain_filter)]
mod soln;

pub fn main() {
    let contents: &str = include_str!("../inputs/day7.txt");
    let part1 = soln::Soln1::part1(contents);
    println!("{:?}", part1);
    let part2 = soln::Soln1::part2(contents);
    println!("{:?}", part2);
    let part1 = soln::Soln1::part1_fast(contents);
    println!("{:?}", part1);
    let part2 = soln::Soln1::part2_fast(contents);
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

        let input: &str = include_str!("../inputs/day7.txt");
        let soln = soln::Soln1::part1(input);
        assert_eq!(soln, 337488);
    }

    #[test]
    fn test_quickselect() {
        assert_eq!(soln::Soln1::median_quickselect(2, vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(soln::Soln1::median_quickselect(1, vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(soln::Soln1::median_quickselect(1, vec![1, 2, 2, 4]), 2);
    }

    #[test]
    fn test_part1_fast() {
        let sample: &str = include_str!("../inputs/sample7.txt");
        let soln = soln::Soln1::part1_fast(sample);
        assert_eq!(soln, 37);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample7.txt");
        let soln = soln::Soln1::part2(sample);
        assert_eq!(soln, 168);

        let input: &str = include_str!("../inputs/day7.txt");
        let soln = soln::Soln1::part2(input);
        assert_eq!(soln, 89647695);
    }

    #[test]
    fn test_part2_fast() {
        let sample: &str = include_str!("../inputs/sample7.txt");
        let soln = soln::Soln1::part2_fast(sample);
        assert_eq!(soln, 168);

        let input: &str = include_str!("../inputs/day7.txt");
        let soln = soln::Soln1::part2_fast(input);
        assert_eq!(soln, 89647695);
    }
}
