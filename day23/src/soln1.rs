use crate::shared::parse;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write,
    iter::repeat,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Soln1 {
    pub grid: Vec<[u8; 3]>,
    pub side_spaces: usize,
}

impl std::fmt::Debug for Soln1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut transposed: Vec<Vec<u8>> = vec![vec![0; self.grid.len()]; 3];
        for ri in 0..self.grid.len() {
            let row = self.grid[ri];
            for ci in 0..row.len() {
                transposed[ci][ri] = self.grid[ri][ci];
            }
        }

        for row in transposed {
            let colchar = |n| match n {
                A => 'A',
                B => 'B',
                C => 'C',
                D => 'D',
                EMPTY => '.',
                INVALID => '#',
                _ => panic!("ahhhh panicc"),
            };
            for char in row {
                f.write_char(colchar(char));
            }
            f.write_char('\n');
        }
        Ok(())
        // f.finish()
        // f.debug_struct("Soln1")
        //     .field("grid", &self.grid)
        //     .field("side_spaces", &self.side_spaces)
        //     .finish()
    }
}

const A: u8 = 0;
const B: u8 = 1;
const C: u8 = 2;
const D: u8 = 3;
const EMPTY: u8 = 4;
const INVALID: u8 = 5;

impl Soln1 {
    pub fn part1(input: &str) -> usize {
        todo!()
    }

    pub fn empty(side_spaces: usize) -> Soln1 {
        let hall = || [EMPTY, INVALID, INVALID];
        let free = || [EMPTY, EMPTY, EMPTY];

        let mut grid: Vec<[u8; 3]> = vec![];
        repeat(hall).take(side_spaces).for_each(|s| grid.push(s()));
        grid.extend(vec![free(), hall(), free(), hall(), free(), hall(), free()].into_iter());
        repeat(hall).take(side_spaces).for_each(|s| grid.push(s()));
        Soln1 { grid, side_spaces }
    }

    pub fn new(cols: [[u8; 2]; 4], side_spaces: usize) -> Soln1 {
        let mut empty = Soln1::empty(side_spaces);
        for col in [side_spaces, side_spaces + 2, side_spaces + 4, side_spaces + 6] {
            // let ch: u8 = cols[]
            let colnum = (col - side_spaces) / 2;
            empty.grid[col][0] = EMPTY;
            empty.grid[col][1] = cols[colnum][0];
            empty.grid[col][2] = cols[colnum][1];
        }
        empty
    }

    pub fn mov(&self, movfrom: (usize, usize), movto: (usize, usize)) -> Soln1 {
        let mut new_board = Soln1 {
            grid: self.grid.clone(),
            side_spaces: self.side_spaces,
        };
        let (a, b) = movfrom;
        let (c, d) = movto;
        let mover = new_board.grid[a][b];
        new_board.grid[a][b] = EMPTY;
        new_board.grid[c][d] = mover;
        new_board
    }

    pub fn one_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let (vr, vc) = from;
        let this = self.grid[vr][vc];
        if vc > 0 && self.grid[vr][vc - 1] == EMPTY {
            moves.push((vr, vc - 1));
        }
        if vc < 2
            && self.grid[vr][vc + 1] == EMPTY
            && (self.grid[vr][1] == EMPTY || self.grid[vr][1] == this)
            && (self.grid[vr][2] == EMPTY || self.grid[vr][2] == this)
        {
            moves.push((vr, vc + 1));
        }
        if vr > 0 && self.grid[vr - 1][vc] == EMPTY {
            moves.push((vr - 1, vc))
        }
        if vr < self.grid.len() - 1 && self.grid[vr + 1][vc] == EMPTY {
            moves.push((vr + 1, vc))
        }
        moves.retain(|(endrow, endcol)| {
            self.grid[*endrow][*endcol] == EMPTY && (*endcol == 2 || self.grid[*endrow][*endcol + 1] != EMPTY)
        });
        moves
    }

    pub fn moves(&self, from: (usize, usize)) {}

    fn cost_one(char: u8) -> usize {
        match char {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
            _ => panic!("no"),
        }
    }

    fn solved(&self) -> bool {
        let mut valid = true;
        for col in [
            self.side_spaces,
            self.side_spaces + 2,
            self.side_spaces + 4,
            self.side_spaces + 6,
        ] {
            let ch: u8 = ((col - self.side_spaces) / 2).try_into().unwrap();
            valid &= self.grid[col][0] == EMPTY;
            valid &= self.grid[col][1] == ch;
            valid &= self.grid[col][2] == ch;
        }
        valid
    }

    fn recurs(
        &self,
        memo: &mut HashMap<Vec<[u8; 3]>, Option<usize>>,
        visiting: &mut HashSet<(Vec<[u8; 3]>)>,
    ) -> Option<usize> {
        println!("visiting:\n{:?}", self);
        if memo.contains_key(&self.grid) {
            return *memo.get(&self.grid).unwrap();
        }
        if self.solved() {
            return Some(0);
        }

        // let starts = vec![];
        let mut best: Option<usize> = None;
        for r in 0..self.grid.len() {
            for c in 0..3 {
                if self.grid[r][c] < EMPTY {
                    let valid_moves = self.one_moves((r, c));
                    for (dr, dc) in valid_moves {
                        let applied = Soln1::mov(&self, (r, c), (dr, dc));
                        if !visiting.contains(&applied.grid) {
                            // let mut new_visiting = visiting.clone();
                            // new_visiting.insert(applied.grid.clone());
                            visiting.insert(applied.grid.clone());
                            // println!("recursing on grid:\n{:?}\nseen:{:?}", applied, memo);
                            let can_solve = applied.recurs(memo, visiting);
                            // memo.insert(applied.grid.clone(), can_solve);
                            if let Some(solve_cost) = can_solve {
                                let new_cost = Self::cost_one(applied.grid[dr][dc]) + solve_cost;
                                if best.is_none() {
                                    best = Some(new_cost);
                                } else {
                                    best = Some(best.unwrap().min(new_cost))
                                }
                            }
                        }
                    }
                }
            }
        }
        memo.insert(self.grid.clone(), best);
        best
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::Soln1;
    use crate::{
        shared::parse,
        soln1::A,
        soln1::C,
        soln1::D,
        soln1::{B, EMPTY},
    };

    #[test]
    fn test_paths() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        let soln = Soln1::empty(2);
        println!("empty:\n{:?}", soln);
        println!("solved?: {}", soln.solved());

        let mut solved = Soln1::empty(0);
        solved.grid[0][2] = A;
        solved.grid[0][1] = A;
        solved.grid[2][2] = B;
        solved.grid[2][1] = B;
        solved.grid[4][2] = C;
        solved.grid[4][1] = C;
        solved.grid[6][1] = D;
        solved.grid[6][2] = D;
        println!("solved grid:\n{:?}", solved);
        println!("solved?: {}", solved.solved());

        let mut soln = Soln1::empty(2);
        soln.grid = vec![
            [0, 5, 5], //HALL
            [4, 5, 5], //HALL
            [4, 4, 4], //COL1
            [1, 5, 5], //HALL
            [4, 4, 3], //COL2
            [1, 5, 5], //HALL
            [4, 4, 4], //COL3
            [3, 5, 5], //HALL
            [4, 4, 0], //COL4
            [2, 5, 5], //HALL
            [2, 5, 5], //HALL
        ];
        println!("test grid:\n{:?}", soln);
        // eprintln!("moves from here:");
        // for r in 0..soln.grid.len() {
        //     for c in 0..3 {
        //         if soln.grid[r][c] < EMPTY {

        //             eprintln!("moves from {:?}", (r, c));
        //             let moves = soln.moves((r, c));
        //             eprintln!("{:?}", moves);
        //         }
        //     }
        // }

        let soln = Soln1::new(parsed, 2);
        println!("sample grid:\n{:?}", soln);
        println!("solved?: {}", soln.solved());
        let mut seen = HashMap::new();
        let mut visiting = HashSet::new();
        let solv = soln.recurs(&mut seen, &mut visiting);
        println!("soln: {:?}", solv);
    }
    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
    }
}
