use shared;
use std::{fmt::Debug, iter, ops::Range};

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str, days: usize) -> usize {
        // 3,4,3,1,2
        let fish = input.trim().split(',').map(|x| x.parse::<u8>().expect("illegal age"));
        let mut fish = fish.collect::<Vec<u8>>();
        for _ in 0..days {
            Self::advance(&mut fish);
        }
        println!("Final state: {:?}", fish);
        fish.len()
    }

    fn advance(fish: &mut Vec<u8>) -> () {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] -= 1;
            }
        }
    }
}
