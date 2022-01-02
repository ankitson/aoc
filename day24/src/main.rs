#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![feature(int_roundings)]
#![feature(asm)]

mod alu;
mod soln1;
use alu::adt::{Instr::*, *};
use alu::alu::*;
use itertools::Itertools;
use std::{fs, str::FromStr, time::Instant};

pub fn main() {
    println!("Hello Day 24!");
    let input: &str = include_str!("../inputs/day24.txt");
    write_cpp(input);

    let mut max = 0;
    let mut min = i64::MAX;
    best_iter(0, 0, 0, vec![], 0, &mut max, &mut min);
    println!("Max: {} Min: {}", max, min);
    println!("verifying max = {}", max);
    let results = prog(
        &max.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect_vec(),
    );
    for (i, (w, x, y, z)) in results.iter().enumerate() {
        println!("After round {}: w={}, x={}, y={}, z={:?}", i + 1, w, x, y, zstack(*z));
    }

    println!("verifying min = {}", min);
    let results = prog(
        &min.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect_vec(),
    );
    for (i, (w, x, y, z)) in results.iter().enumerate() {
        println!("After round {}: w={}, x={}, y={}, z={:?}", i + 1, w, x, y, zstack(*z));
    }

    // let mut alu = ALU::default();
    // let out = run_with_input(
    //     &mut alu,
    //     Instrs::from_str(input).unwrap(),
    //     prog_input1.into_iter().map(|x| x.try_into().unwrap()).collect_vec(),
    // );
    // println!("Simulation output: {:?}", out);
}

fn zstack(mut z: i64) -> Vec<i64> {
    let mut vals = vec![];
    while z > 0 {
        vals.push(z % 26);
        z /= 26;
    }
    vals
}

fn best_iter(mut x: i64, y: i64, mut z: i128, ws: Vec<i64>, i: usize, max: &mut i64, min: &mut i64) {
    // println!("best iter {}", i);
    let xadds = [10, 13, 15, -12, 14, -2, 13, -12, 15, 11, -3, -13, -12, -13];
    let yadds = [10, 5, 12, 12, 6, 4, 15, 3, 7, 11, 2, 12, 4, 11];
    let zdivs = [1, 1, 1, 26, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26];

    let rem_pops = *&zdivs[i..].iter().filter(|x| **x == 26).count();
    let zsta = zstack(z.try_into().unwrap());
    if zsta.len() > rem_pops {
        // println!("early terminate stack = {:?} at {} but {} pops", zsta, i, rem_pops);
        return;
    }

    if i == 14 {
        if z == 0 {
            let vstr = ws
                .iter()
                .map(|d| char::from_digit((*d).try_into().unwrap(), 10).unwrap())
                .collect::<String>();
            let n = vstr.parse::<i64>().unwrap();
            if n > *max {
                *max = n;
            }
            if n < *min {
                *min = n;
            }
            // println!("success with: {:?}", ws);
        } else {
            // println!("fail ")
        }
        return;
    }
    x = ((z % 26) as i64) + xadds[i];
    z /= zdivs[i];

    if (x >= 1 && x <= 9 && i >= 15) {
        let w = x;
        // println!("greedy pick w[{}] = {} with ws = {:?}", i, w, ws);
        z = ((w + yadds[i]) as i128) + (26 * z);
        let mut wsn = ws.clone();
        wsn.push(w);
        best_iter(x, y, z, wsn, i + 1, max, min)
    } else {
        let zcopy = z;
        for j in 1..=9 {
            z = zcopy;
            let w = j;
            if x != w {
                z = ((w + yadds[i]) as i128) + (26 * z);
            }
            let mut wsn = ws.clone();
            wsn.push(w);
            best_iter(x, y, z, wsn, i + 1, max, min)
        }
    }
}

#[rustfmt::skip]
fn iter(w: i64, mut x: i64, y: i64, mut z: i64, i: usize) -> (i64, i64, i64, i64) {
    //                     1   2   3    4   5   6   7    8   9  10  11   12   13   14
    let xadds = [10, 13, 15, -12, 14, -2, 13, -12, 15, 11, -3, -13, -12, -13];
    let yadds = [10,  5, 12,  12,  6,  4, 15,   3,  7, 11,  2,  12,   4,  11];
    let zdivs = [ 1,  1,  1,  26,  1, 26,  1,  26,  1,  1, 26,  26,  26,  26];
    x = (z % 26) + xadds[i]; //x[i] = peek(z) + xadds[i] = w+yadds[i-1]+ xadds[i] IF zdiv[i-1] = 1 AND x[i-1] != input[i-1]
                             //x = peek(z) + xadds[i] =
    z /= zdivs[i];
    if x != w {
        z = (w + yadds[i]) + (26 * z); //zdiv=1, z = 26*z + (w+yadds[i]). yadds+w <= 25 (max w = 10, max yadd = 15)
    }
    (w, x, y, z)
}

fn prog(inputs: &[i64]) -> Vec<(i64, i64, i64, i64)> {
    let mut w = 0i64;
    let mut x = 0i64;
    let mut y = 0i64;
    let mut z = 0i64;

    let mut results = vec![];
    for i in 0..14 {
        w = inputs[i];
        (w, x, y, z) = iter(w, x, y, z, i);
        results.push((w, x, y, z));
    }
    results
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

fn read_inputs(path: &str) -> Vec<i64> {
    let inp_str = std::fs::read_to_string(path).unwrap();
    let inputs = inp_str.lines().map(|c| c.parse::<i64>().unwrap()).collect_vec();
    inputs
}

fn run_with_input(alu: &mut ALU, instrs: Instrs, input: Vec<isize>) -> Vec<isize> {
    alu.eval(instrs, input);
    alu.state()
}

fn run_with_file(alu: &mut ALU, prog: &str, input_path: &str) -> Vec<isize> {
    let instrs = Instrs::from_str(prog).unwrap();
    run_with_input(
        alu,
        instrs,
        read_inputs(input_path)
            .into_iter()
            .map(|x| x.try_into().unwrap())
            .collect_vec(),
    )
}

fn write_cpp(prog: &str) {
    let instrs = Instrs::from_str(prog).unwrap();
    let compiled = soln1::Soln1::compile_cpp(instrs.clone());
    fs::write("prog.cpp", compiled).expect("unable to write");
}
