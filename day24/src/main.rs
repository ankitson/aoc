#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]
#![feature(asm)]

mod alu;
mod soln1;
use alu::adt::Register::*;
use alu::adt::{Instr::*, *};
use alu::alu::*;
use itertools::Itertools;
use std::fs::{self, File};
use std::str::FromStr;
use std::time::Instant;
use util::combo;

pub fn main() {
    println!("Hello Day 24!");
    let prog: &str = include_str!("../inputs/day24.txt");
    write_cpp(&prog);
    let mut alu = ALU::default();
    run_with_file(&mut alu, &prog, "/home/ankit/code/aoc-2021/day24/inputs/inp1.txt");

    let instrs = Instrs::from_str(prog).unwrap();
    // println!("{:?}", instrs.lifetimes());
}

fn bruteforce(input: &str) {
    let instrs = Instrs::from_str(input).unwrap();
    let mut alu = ALU::default();
    let expected_input_len = instrs.0.iter().filter(|instr| matches!(**instr, Inp { .. })).count();

    let mut successes = vec![];
    let istart: u64 = 99_999_999_999_999u64;
    let ilimit: u64 = 11_111_111_111_111u64;
    let mut i: u64 = istart;

    let clock_start = Instant::now();
    while i >= ilimit {
        let ichars = format!("{:0<14}", i);
        if ichars.contains('0') {
            if i % 10000 == 0 {
                let elapsed = clock_start.elapsed().as_millis();
                println!("Found {}/{} inputs in {}ms ", successes.len(), istart - i, elapsed);
                println!("Last: {} = {:?}", i + 1, alu.state());
                if !successes.is_empty() {
                    println!("succesful inputs so far: {:?}", successes);
                }
            }
            i -= 1;
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

        i -= 1;
    }
}

fn run_with_input(alu: &mut ALU, instrs: Instrs, input: Vec<isize>) -> Vec<isize> {
    alu.eval(instrs, input);
    alu.state()
}

fn run_with_file(alu: &mut ALU, prog: &str, input_path: &str) -> Vec<isize> {
    let instrs = Instrs::from_str(prog).unwrap();

    let inp_str = std::fs::read_to_string(input_path).unwrap();
    let input_vec = inp_str.lines().map(|c| c.parse::<isize>().unwrap()).collect_vec();
    let final_state = run_with_input(alu, instrs, input_vec);
    println!("output = {:?}", final_state);
    final_state
}

fn write_cpp(prog: &str) {
    let instrs = Instrs::from_str(prog).unwrap();
    let compiled = soln1::Soln1::compile_cpp(instrs.clone());
    fs::write("prog.cpp", compiled).expect("unable to write");
}
