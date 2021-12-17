use crate::shared::{parse_bv, parse_packet};

// use crate::shared::parse;
// use itertools::Itertools;

pub struct Soln1 {}

impl Soln1 {
    pub fn part1(input: &str) -> u64 {
        static mut vcounter: u64 = 0u64;
        let bv = parse_bv(input);
        unsafe {
            let (rem, parsed) = parse_packet(&bv, &mut vcounter).unwrap();
            println!("parsed literal: {:b}", parsed);
            println!("rem: {:?}", rem.len());
            return vcounter;
        }
    }

    pub fn part2(input: &str) -> u64 {
        static mut vcounter: u64 = 0u64;
        let bv = parse_bv(input);
        unsafe {
            let (rem, evaled) = parse_packet(&bv, &mut vcounter).unwrap();
            return evaled;
        }
    }
}
