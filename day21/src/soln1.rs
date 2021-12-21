use crate::shared::parse;
use itertools::{iproduct, izip, Itertools};
use rayon::prelude::*;
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let mut poses = parse(input);
        let num_players = poses.len();
        let mut scores = vec![0; num_players];
        let mut die = 1;
        let mut nturns = 0;

        loop {
            let player = nturns % num_players;
            poses[player] = Self::incr_pos(poses[player], 3 * die + 3);
            scores[player] += poses[player];
            nturns += 1;
            die += 3;
            if scores[player] >= 1000 {
                let losing_score = scores[1 - player];
                return losing_score * nturns * 3;
            }
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

    fn step_n(
        starts: Vec<usize>,
        rolls: usize,
        mut roll_sub: usize,
        scores: &mut [usize],
        wins1: &mut u64,
        wins2: &mut u64,
    ) {
        let turn = (42 - rolls) % 2; //at 21 rolls remaining, its p1s turn.

        if scores[1 - turn] > 21 {
            if 1 - turn == 0 {
                *wins1 += 1;
            } else {
                *wins2 += 1;
            }
            return;
        }

        let prefix: String = ".".repeat(42 - rolls);
        println!("{} {}, {}", prefix, scores[0], scores[1]);
        let mut next_rolls = rolls;
        if roll_sub == 1 {
            next_rolls -= 1;
            roll_sub = 3;
        };
        for i in 0..roll_sub {
            for roll in 1..=3 {
                let mut new_starts = starts.clone();
                new_starts[turn] = Self::incr_pos(starts[turn], roll);
                let mut new_scores = &mut scores[..]; //scores.clone();
                new_scores[turn] += new_starts[turn];

                Self::step_n(new_starts, next_rolls, roll_sub - 1, new_scores, wins1, wins2)
            }
        }
    }
    
    pub fn recurse(
        p1pos: usize,
        p2pos: usize,
        p1score: usize,
        p2score: usize,
        p1wins: &mut u64,
        p2wins: &mut u64,
        turn: (usize, usize),
        depth: usize,
        longest: &mut usize,
        mut last_rolls: Vec<usize>
    ) {
        let (pturn, num_rolls_left) = turn;
        if depth > *longest {
            *longest = depth;
        }
        if p1score >= 21 {
            *p1wins += 1;
            return;
        } else if p2score >= 21 {
            *p2wins += 1;
            return;
        }
        if p1score >= 21 && p2score >= 21 {
            panic!("illegal state??")
        }

        // (1..=3).par_map(|roll| 
        for roll in 1..=3 {
            if pturn == 0 {
                //p1s turn
                let nextpos = if num_rolls_left == 1 { Self::incr_pos(Self::incr_pos(Self::incr_pos(p1pos, last_rolls[0]),last_rolls[1]),roll) } else { p1pos };
                let nextscore = if num_rolls_left == 1 { p1score + nextpos } else { p1score };
                let next_turn = if num_rolls_left == 1 { 1 - pturn } else { pturn };
                let next_rolls = if num_rolls_left == 1 { 3 } else { num_rolls_left - 1 };
                let next_last_rolls = if num_rolls_left == 1 { vec![] } else { let mut nr = last_rolls.clone(); nr.push(roll); nr };
                let prefix = ".".repeat(depth + 1);
                // println!(
                //     "{}P1 rolled {} . Score {} . Next turn {},{}",
                //     prefix, roll, nextscore, next_turn, next_rolls
                // );
                Self::recurse(
                    nextpos,
                    p2pos,
                    nextscore,
                    p2score,
                    p1wins,
                    p2wins,
                    (next_turn, next_rolls),
                    depth + 1,
                    longest,
                    next_last_rolls
                )
            } else {
                let nextpos = if num_rolls_left == 1 { Self::incr_pos(Self::incr_pos(Self::incr_pos(p1pos, last_rolls[0]),last_rolls[1]),roll) } else { p2pos };
                let nextscore = if num_rolls_left == 1 { p1score + nextpos } else { p1score };
                // let nextpos = Self::incr_pos(p2pos, roll);
                let next_turn = if num_rolls_left == 1 { 1 - pturn } else { pturn };
                let next_rolls = if num_rolls_left == 1 { 3 } else { num_rolls_left - 1 };
                let next_last_rolls = if num_rolls_left == 1 { vec![] } else { let mut nr = last_rolls.clone(); nr.push(roll); nr };
                let prefix = ".".repeat(depth + 1);
                // println!(
                //     "{}P2 rolled {} . Score {} . Next turn {},{}",
                //     prefix, roll, nextscore, next_turn, next_rolls
                // );

                Self::recurse(
                    p1pos,
                    nextpos,
                    p1score,
                    nextscore,
                    p1wins,
                    p2wins,
                    (next_turn, next_rolls),
                    depth + 1,
                    longest,
                    next_last_rolls
                )
            }
        }
    }

    pub fn part2(input: &str) -> (u64, u64) {
        let mut poses = parse(input);
        let mut wins1 = 0u64;
        let mut wins2 = 0u64;
        Self::recurse(
            poses[0],
            poses[1],
            0,
            0,
            &mut wins1,
            &mut wins2,
            (0, 3),
            0,
            &mut 0,
            vec![]
        );
        (wins1, wins2)
        //Self::step_n(poses, 42, 3, &mut [0; 2], &mut wins1, &mut wins2);
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

        println!("Part 2 (sample) = {:?}", Soln1::part2(sample));
    }
}
