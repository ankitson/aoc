use crate::shared::parse;
use itertools::{iproduct, izip, Itertools};
use rayon::prelude::*;
use std::{sync::atomic::{AtomicU64, Ordering}, collections::HashMap};

pub struct Soln1 {
    pub cache: HashMap<(usize,usize,usize,usize,(usize,usize),usize),(u64,u64)>
}
impl Soln1 {
    pub fn new(cache: HashMap<(usize,usize,usize,usize,(usize,usize),usize),(u64,u64)>) -> Self { Self { cache } }

    // pub fn default() -> Self {
        // Soln1 { cache: &'static mut HashMap::new() }
    // }
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

    fn incr_pos(pos: usize, roll: usize) -> usize {
        // let mut posi: isize = pos.try_into().unwrap();
        // let rolli: isize = roll.try_into().unwrap();
        // posi -= 1;
        ((pos + roll + 10 - 1) % 10) + 1
        // posi += 1;
        // posi.try_into().unwrap()


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
        &mut self,
        p1pos: usize,
        p2pos: usize,
        p1score: usize,
        p2score: usize,
        turn: (usize, usize),
        depth: usize,
        last_rolls: usize,
        target: usize,
    ) -> (u64, u64) {
        let (pturn, num_rolls_left) = turn;
        
        println!("recurse: {} {} {} {} {:?} {}",p1pos,p2pos,p1score,p2score,turn,last_rolls);
        //10*10*20*20*2 = 80000 unique kys
        // if (self.cache.contains_key(&(p1pos,p2pos,p1score,p2score,turn,last_rolls))) {
        //     let val = *self.cache.get(&(p1pos,p2pos,p1score,p2score,turn,last_rolls)).unwrap();
        //     println!("cache hit for {:?} = {:?}", (p1pos,p2pos,p1score,p2score,turn,last_rolls), val);
        //     return val;
        // }

        if p1score >= target {
            return (1,0);
        } else if p2score >= target {
            return (0,1);
        }
        if p1score >= target && p2score >= target {
            panic!("illegal state??")
        }


        let mut p1w = 0u64;
        let mut p2w = 0u64;
        (1..=3).for_each(|roll| {
            if pturn == 0 {
                //p1s turn
                let nextpos = if num_rolls_left == 0 { Self::incr_pos(p1pos, last_rolls+roll) } else { p1pos };
                let nextscore = if num_rolls_left == 0 { p1score + nextpos } else { p1score };
                // println("p1 scores {} after landing at {}")
                let next_turn = if num_rolls_left == 0 { 1 - pturn } else { pturn };
                let next_rolls = if num_rolls_left == 0 { 3 } else { num_rolls_left - 1 };
                let next_last_rolls = if num_rolls_left == 0 { 0 } else { last_rolls + roll };
                let prefix = ".".repeat(depth + 1);
                // println!(
                //     "{}P1 rolled {} . Score {} . Next turn {},{}",
                //     prefix, roll, nextscore, next_turn, next_rolls
                // );
                
                let (p1r, p2r) = self.recurse(
                    nextpos,
                    p2pos,
                    nextscore,
                    p2score,
                    (next_turn, next_rolls),
                    depth + 1,
                    next_last_rolls,
                    target
                );
                p1w += p1r;
                p2w += p2r;
            } else {
                let nextpos = if num_rolls_left == 0{ Self::incr_pos(p2pos, last_rolls+roll) } else { p2pos };
                let nextscore = if num_rolls_left == 0 { p1score + nextpos } else { p1score };
                // let nextpos = Self::incr_pos(p2pos, roll);
                let next_turn = if num_rolls_left == 0 { 1 - pturn } else { pturn };
                let next_rolls = if num_rolls_left == 0 { 3 } else { num_rolls_left - 1 };
                let next_last_rolls = if num_rolls_left == 0 { 0 } else { last_rolls + roll };
                let prefix = ".".repeat(depth + 1);
                // println!(
                //     "{}P2 rolled {} . Score {} . Next turn {},{}",
                //     prefix, roll, nextscore, next_turn, next_rolls
                // );

                let (p1r,p2r) = self.recurse(
                    p1pos,
                    nextpos,
                    p1score,
                    nextscore,
                    (next_turn, next_rolls),
                    depth + 1,
                    next_last_rolls,
                    target
                );
                p1w += p1r;
                p2w += p2r;
                
            }
        });
        self.cache.insert((p1pos,p2pos,p1score,p2score,turn,last_rolls), (p1w,p2w));
        (p1w, p2w)
    }

    pub fn part2(&mut self, input: &str, target: usize) -> (u64, u64) {
        let mut poses = parse(input);
        let wins1 = AtomicU64::new(0);
        let wins2 = AtomicU64::new(0);
        // let mut wins1 = 0u64;
        // let mut wins2 = 0u64;
        // let mut memo = HashMap::new();
        let (a,b) = self.recurse(
            poses[0],
            poses[1],
            0,
            0,
            (0, 3),
            0,
            0,
            target
        );
        (a, b)
        //Self::step_n(poses, 42, 3, &mut [0; 2], &mut wins1, &mut wins2);
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_sample() {
        let sample: &str = include_str!("../inputs/sample.txt");
        assert_eq!(Soln1::part1(sample), 739785);

        let cache = HashMap::new();
        let mut soln1 = Soln1::new(cache);
        println!("Part 2 (sample) = {:?}", soln1.part2(sample));

        // println!("{}", Soln1::part22(sample));
    }
}
