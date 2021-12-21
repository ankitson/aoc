use crate::shared::parse;
use itertools::Itertools;
use std::iter::repeat;

pub struct Soln1 {}
impl Soln1 {
    fn next_position(mut position: usize, die: usize, circle_size: usize) -> usize {
        let incr = die + die + 1 + die + 2;
        position -= 1;
        position += incr;
        position %= circle_size;
        position += 1;
        position
    }
    pub fn part1(input: &str) -> usize {
        let mut poses = parse(input);
        let num_players = poses.len();
        let mut scores = vec![0; num_players];
        let mut player = 0;
        let mut die = 1;
        let mut nturns = 0;

        loop {
            poses[player] = Self::next_position(poses[player], die, 10);
            scores[player] += poses[player];
            nturns += 1;
            die += 3;
            if scores[player] >= 1000 {
                let losing_score = scores[1 - player];
                return losing_score * nturns * 3;
            }
            player = (player + 1) % num_players;
        }
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        assert_eq!(Soln1::part1(sample), 739785);
    }
}
