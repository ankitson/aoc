#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]

mod alu;
mod soln1;
use alu::adt::{Instr::*, *};
use alu::alu::*;
use itertools::Itertools;
use std::str::FromStr;
use std::time::Instant;
use util::combo;

pub fn main() {
    println!("Hello Day 24!");
    let day24: &str = include_str!("../inputs/day24.txt");

    bruteforce(day24);
}

fn bruteforce(input: &str) {
    let instrs = Instrs::from_str(input).unwrap();
    let mut alu = ALU::default();
    let expected_input_len = instrs.0.iter().filter(|instr| matches!(**instr, Inp { .. })).count();

    let valid_digits = (1..9).collect_vec();

    let istart: u64 = 11_111_111_111_111u64;
    let mut i: u64 = istart;
    let mut limit: u64 = 99_999_999_999_999;

    let clock_start = Instant::now();

    let mut successes = vec![];
    while i < limit {
        let ichars = format!("{:0<14}", i);
        if ichars.contains('0') {
            if i % 10000 == 0 {
                let elapsed = clock_start.elapsed().as_millis();
                println!("Found {}/{} inputs in {}ms ", successes.len(), i - istart, elapsed);
                println!("Last: {} = {:?}", i - 1 - istart, alu.state());
                if !successes.is_empty() {
                    println!("succesful inputs so far: {:?}", successes);
                }
            }
            i += 1;
            continue;
        }
        let idigs: Vec<isize> = ichars
            .chars()
            .map(|c| {
                if c == 'X' {
                    0isize
                } else {
                    c.to_digit(10).unwrap().try_into().unwrap()
                }
            })
            .collect_vec();
        alu.reset();
        run_with_input(&mut alu, instrs.clone(), idigs);
        let state = alu.state();
        if state[3] == 0 {
            println!("ALU halted successfully on {:?}! State = {:?}", input, state);
            successes.push(input);
        }

        i += 1;
    }
}

fn run_with_input(alu: &mut ALU, instrs: Instrs, input: Vec<isize>) -> Vec<isize> {
    alu.eval(instrs, input);
    alu.state()
}
