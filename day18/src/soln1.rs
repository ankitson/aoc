use crate::shared::parse;
use itertools::iproduct;

type FN = Vec<(usize, usize)>;
pub struct Soln1 {}
impl Soln1 {
    fn reduce(fishnum: FN) -> FN {
        todo!()
    }

    pub fn reduce_full(mut fishnum: FN) -> FN {
        let mut reduced = Self::reduce(fishnum);
        while fishnum != reduced {
            fishnum = reduced;
            reduced = Self::reduce(reduced);
        }
        reduced
    }

    pub fn magnitude(fishnum: FN) -> usize {
        todo!()
    }

    pub fn add(l: &mut FN, r: &FN) -> () {
        l.extend(r)
    }

    pub fn part1(input: &str) -> usize {
        let mut parsed = parse(input);
        let mut accum = parsed[0].clone();
        for num in &parsed[1..] {
            Self::add(&mut accum, num)
        }
        println!("Sum: {:?}", accum);
        let mut reduced = Self::reduce_full(accum);
        Self::magnitude(reduced)
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::identity_op)]
    // use day17::{shared, soln1};
    // use day17::Soln1::trajectory;
    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = Soln1::part1(sample);
        // println!("Part 1 = {:?}", part1);
        // assert_eq!(part1, 45);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = Soln1::part2(sample);
        // println!("Part 2 = {:?}", part2);
        // assert_eq!(part2, 112);
    }
}
