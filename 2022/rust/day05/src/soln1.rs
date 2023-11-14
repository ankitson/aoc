

use crate::shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> String {
        Self::part1_core(shared::parse(input))
    }

    pub fn part1_core(input: (Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
        let (mut stacks,moves) = input;
        // println!("{:?} {:?}", stacks, moves);
        for (count, from, to) in &moves {
            let fromstack = stacks[*from].clone();
            stacks[*to].reverse();
            stacks[*to].extend_from_slice(&fromstack[0..*count]);
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

    pub fn part2(input: &str) -> String {
        Self::part2_core(shared::parse(input))
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
