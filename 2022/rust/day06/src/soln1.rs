use std::collections::{HashSet, VecDeque};



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
                for j in i + 1..seen.len() {
                    if seen[i] == seen[j] {
                        dup = true;
                    }
                }
            }
            if !dup {
                return (i + 1).try_into().unwrap();
            }
        }
        panic!("AHHH");
    }

    pub fn part2(input: &str) -> i32 {
        Self::part2_core(shared::parse(input))
    }

    pub fn part2_core(input: &str) -> i32 {
        let mut seen: VecDeque<char> = VecDeque::from([]);
        for i in 0..input.len() {
            let char = input.chars().nth(i).unwrap();
            if seen.len() < 14 {
                seen.push_back(char);
                continue;
            }
            seen.pop_front().unwrap();
            seen.push_back(char);
            let mut dup = false;
            for i in 0..seen.len() {
                for j in i + 1..seen.len() {
                    if seen[i] == seen[j] {
                        dup = true;
                    }
                }
            }
            if !dup {
                return (i + 1).try_into().unwrap();
            }
        }
        panic!("AHHH");
    }

    pub fn part2_windows(input: &str) -> usize {
        input.as_bytes().windows(14).position(|w| -> bool { HashSet::<_>::from_iter(w.iter()).len() == 14 }).unwrap()
            + 14
    }

    #[allow(dead_code)]
    pub fn part2_set(input: &str) -> i32 {
        let mut seen: VecDeque<char> = VecDeque::from([]);
        let mut seen_set: HashSet<char> = HashSet::with_capacity(14);
        for i in 0..input.len() {
            let char = input.chars().nth(i).unwrap();
            if seen.len() < 14 {
                seen.push_back(char);
                seen_set.insert(char);
                continue;
            }
            if seen_set.len() == 14 {
                return (i + 1).try_into().unwrap();
            }
            println!("ch = {} setlen = {}", i, seen_set.len());
            let oldest = seen.pop_front().unwrap();
            seen.push_back(char);
            //BUG: this will always remove oldest from the set, even if it reoccurs later in the seen vector
            //so the set ends up being smaller than it should.
            seen_set.remove(&oldest);
            seen_set.insert(char);
        }
        panic!("AHHH");
    }
}
