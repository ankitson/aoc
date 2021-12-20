use crate::shared::parse;
use itertools::iproduct;
pub struct Soln1 {}
impl Soln1 {
   
    pub fn part1(input: &str) -> usize {
        
        todo!()
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = Soln1::part1(sample);
        println!("Part 1 (sample1) = {:?}", part1);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample2.txt");
        let part2 = Soln1::part2(sample);
        println!("Part 2 (sample2) = {:?}", part2);
    }
}
