use std::collections::VecDeque;

use itertools::Itertools;

use crate::shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> i32 {
        Self::part1_core(shared::parse(input))
    }

    pub fn part1_core(input: &str) -> i32 {
        let mut seen: VecDeque<char> = VecDeque::from([]);
        for i in 0..input.len() {
            let char = input.chars().nth(i).unwrap();
            if seen.len() < 4 {
                seen.push_back(char);
                continue;
            }
            seen.pop_front();
            seen.push_back(char);
            let mut dup = false;
            for i in 0..seen.len() {
                for j in i+1..seen.len() {
                    if seen[i] == seen[j] {
                        dup = true;
                    }
                }
            }
            if !dup {
                return (i+1).try_into().unwrap();
            }
        }
        panic!("AHHH"); 
    }

    pub fn part2(input: &str) -> String {
        unimplemented!();
        // Self::part2_core(shared::parse(input))
    }

    pub fn part2_core(input: (Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
        let (mut stacks,moves) = input;
        // println!("{:?} {:?}", stacks, moves);
        for (count, from, to) in &moves {
            let fromstack = stacks[*from].clone();
            stacks[*to].reverse();
            let rev = fromstack.iter().take(*count).rev();
            stacks[*to].extend(rev);
            stacks[*to].reverse();

            stacks[*from].reverse();
            let orglen = stacks[*from].len();
            stacks[*from].truncate(orglen-count);
            stacks[*from].reverse();
            // println!("{:?} {:?}", stacks, moves);
        }
        // println!("{:?} {:?}", stacks, moves);
        let mut top = String::new();
        for stack in stacks {
            top.push(stack[0]);
        }
        top
    }
}
