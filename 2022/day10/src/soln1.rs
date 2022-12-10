// use std::collections::HashSet;
use itertools::Itertools;
use crate::shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> i32 {
        let cycles = [20,60,100,140,180,220];
        let mut cycle = 1;
        let mut x = 1;
        let mut strength = 0;
        for line in input.lines() {
            let parts = line.split(" ").collect_vec();
            match parts[0] {
                "noop" => {
                    if cycles.contains(&cycle) {
                        println!("noop value at cycle {} = {}", cycle, x);
                        strength += cycle * x;
                    } 
                    cycle += 1;
                    // println!("nop cycle = {}", cycle);
                    // if cycles.contains(&cycle) {
                        // println!("value at cycle {} / {} = {}", cycle, cycle+1, x);
                        // strength += cy/cle * x;
                    // }
                },
                "addx" => {
                    //addx 19
                    // if cycles.contains(&cycle) {
                        // println!("value at cycle {} / {} = {}", cycle, cycle+1, x);
                        // strength += cycle * x;
                    // }
                    
                    //cycle is 18 before add => cycle is 20 before next instruction
                    //cycle is 19, 20 before add => use current X
                    if cycles.contains(&cycle) {
                        println!("add1 value at cycle {} = {}", cycle, x);
                        strength += cycle * x;
                    } else if cycles.contains(&(cycle+1)) {
                        println!("add2 value at cycle {} = {}", cycle+1, x);

                        strength += (cycle+1) * x;
                    }
                    
                    cycle += 2;
                    // println!("add cycle = {}", cycle);
                    let n = parts[1].parse::<i32>().expect("ah");
                    x += n;
                    // if cycles.contains(&cycle) {
                        // println!("value at cycle {} / {} = {}", cycle, cycle+1, x);
                        // strength += cycle * x;
                    // }
                },
                _      => panic!("illegal")
            }
        }
        // if cycles.contains(&cycle) || cycles.contains(&(cycle+1)) {
            // println!("value at cycle {} / {} = {}", cycle, cycle+1, x);
            // strength += cycle * x;
        // }
        strength
        // todo!()
    }

    pub fn part1_core(grid: Vec<Vec<u32>>) -> i32 {
        todo!()
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }

    pub fn part2_core(grid: Vec<Vec<u32>>) -> i32 {
        todo!()
    }
}
