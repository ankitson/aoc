use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        println!("{}", input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let mut sands = 0;
        loop {}
        todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
