use itertools::Itertools;

use std::mem;

pub struct Soln1 {}
impl Soln1 {    
    //ChatGPT
    fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
        let mut result = vec![];
        for line in input.split('\n') {
            if (line.is_empty()) { continue; }
            let mut numbers = line.split(',');
            let first = numbers.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect_tuple().unwrap();
            let second = numbers.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect_tuple().unwrap();
            result.push((first, second));
        }
        result
    }
    
    pub fn part1(input: &str) -> u32 {
        let tups = Self::parse_input(input);
        let mut count = 0;
        for (mut r1,mut r2) in &tups {
            if r1.0 > r2.0 || r2.1 > r1.1 { 
                mem::swap(&mut r1, &mut r2);
            }
            
            if (r1.0 <= r2.0 && r1.1 >= r2.1) { // || (r2.0 <= r1.0 && r2.1 >= r1.1) {
                // println!("r2={:?} is contained in r1={:?}", r2, r1);
                count += 1;
            }
        }
        count
    }

    pub fn part2(input: &str) -> u32 {
        let tups = Self::parse_input(input);
        let mut count = 0;
        for (mut r1, mut r2) in &tups {
            if r1.0 > r2.0 { 
                mem::swap(&mut r1, &mut r2);
            }
            if (r2.0 <= r1.1) { count += 1}
        }
        count
    }
}
