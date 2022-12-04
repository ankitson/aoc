use itertools::Itertools;

use std::{mem, ops::RangeBounds};

pub struct Soln1 {}
impl Soln1 {
    // pub fn parse(input: &str) -> (Vec<u8>, Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]>) {
    //     let mut lines = input.split('\n').filter(|x| !x.is_empty()).collect::<Vec<&str>>();

    //     let mut boards: Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

    //     let moves: Vec<u8> = lines[0]
    //         .split(',')
    //         .map(|x| x.parse::<u8>().expect("unable to parse num"))
    //         .collect::<Vec<u8>>();

    //     lines.remove(0);

    //     for board_lines in lines.chunks(BOARD_SIZE) {
    //         let mut board: [[i8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
    //         for i in 0..BOARD_SIZE {
    //             let line = board_lines[i];
    //             let nums: Vec<i8> = line
    //                 .split_whitespace()
    //                 .map(|x| x.parse::<i8>().unwrap_or_else(|x| panic!("unable to parse num {}", x)))
    //                 .clone()
    //                 .collect();
    //             board[i][..BOARD_SIZE].clone_from_slice(&nums[..BOARD_SIZE]);
    //         }
    //         boards.push(board);
    //     }

    //     (moves, boards)
    // }

    // pub fn unparse(output: (u32, u32)) -> String {
    //     (output.0 * output.1).to_string()
    // }
    
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
            if r1.0 > r2.0 { 
                mem::swap(&mut r1, &mut r2);
            }
            
            if (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1) {
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
