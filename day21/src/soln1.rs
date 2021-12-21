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

    fn incr_pos(mut pos: usize, roll: usize) -> usize {
        let mut posi: isize = pos.try_into().unwrap();
        let rolli: isize = roll.try_into().unwrap();
        posi -= 1;
        posi = (posi + rolli) % 10;
        posi += 1;
        posi.try_into().unwrap()
    }

    fn step_n(starts: Vec<usize>, rolls: usize, scores: Vec<usize>, wins1: &mut u64, wins2: &mut u64) {
        let turn = (21 - rolls) % 2; //at 21 rolls remaining, its p1s turn.

        for roll in 1..4 {
            let mut new_starts = starts.clone();
            new_starts[turn] = Self::incr_pos(starts[turn], roll);
            let mut new_scores = scores.clone();
            new_scores[turn] += new_starts[turn];
            if new_scores[turn] > 21 {
                if turn == 0 {
                    *wins1 += 1;
                } else {
                    *wins2 += 1;
                }
                return;
            }
            Self::step_n(new_starts, rolls - 1, new_scores, wins1, wins2)
        }
    }

    pub fn part2(input: &str) -> (u64, u64) {
        let mut poses = parse(input);
        let np = poses.len();
        let mut wins1 = 0u64;
        let mut wins2 = 0u64;
        Self::step_n(poses, 21, vec![0; np], &mut wins1, &mut wins2);
        (wins1, wins2)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_sample() {
        let sample: &str = include_str!("../inputs/sample.txt");
        assert_eq!(Soln1::part1(sample), 739785);

        println!("{:?}", Soln1::part2(sample));
    }
}
