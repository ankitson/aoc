mod soln;

pub fn main() {
    let contents: &str = include_str!("../inputs/day6.txt");
    let part1 = soln::Soln1::part1(contents, 80);
    println!("{:?}", part1);
    let part2 = soln::Soln1::part2(contents, 256);
    println!("{:?}", part2);
    // let part2 = soln::Soln1::part2(contents, 1000);
    // println!("{:?}", part2);

    // let polysoln = soln::Soln1::part2_faster(contents, 80);
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use crate::soln::{self, SquareMatrix};

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

    #[test]
    fn test_mat_multiply() {
        let m1 = SquareMatrix::<3>([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let m2 = SquareMatrix::<3>([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let m3 = m1.mul(m2);
        let expected = SquareMatrix::<3>([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);
        assert_eq!(m3, expected);
    }
}
