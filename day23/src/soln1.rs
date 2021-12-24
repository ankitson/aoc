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
    pub fn part1(input: &str) -> Option<usize> {
        let parsed = parse(input);
        let soln = Soln1::new(parsed, 2);
        // println!("sample grid:\n{:?}", soln);
        // println!("solved?: {}", soln.solved());
        let mut seen = HashMap::new();
        let mut visiting = HashMap::new();
        soln.recurs(&mut seen, &mut visiting, 0, 0)
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

    pub fn moves(&self, from: (usize, usize)) -> Vec<(usize, usize, usize)> {
        // let mut moves = vec![]; //(dest,dist)
        let (vc, vr) = from;
        let ch = self.grid[vc][vr];
        let col_headers = vec![
            self.side_spaces,
            self.side_spaces + 2,
            self.side_spaces + 4,
            self.side_spaces + 6,
        ];

        let nudge_horiz = |(ci, ri): (usize, usize)| {
            let col_headers = vec![
                self.side_spaces,
                self.side_spaces + 2,
                self.side_spaces + 4,
                self.side_spaces + 6,
            ];
            let mut nudges: Vec<(usize, usize, usize)> = vec![];
            if ri == 0 {
                let mut dist = 0;
                if ci > 0 {
                    let mut lci: usize = ci - 1;
                    while (self.grid[lci][ri] == EMPTY) {
                        dist += 1;
                        if !col_headers.contains(&lci) {
                            nudges.push((lci, ri, dist))
                        }
                        if (lci == 0) {
                            break;
                        }
                        lci -= 1;
                    }
                }
                dist = 0;
                let mut rci: usize = ci + 1;
                while (rci < self.grid.len() && self.grid[rci][ri] == EMPTY) {
                    dist += 1;
                    if !col_headers.contains(&rci) {
                        nudges.push((rci, ri, dist))
                    }
                    rci += 1;
                }
            }
            nudges
        };

        let right_col = |(vc, vr): (usize, usize)| {
            let ch = self.grid[vc][vr];
            (ch == A && vc == col_headers[0])
                || (ch == B && vc == col_headers[1])
                || (ch == C && vc == col_headers[2])
                || (ch == D && vc == col_headers[3])
        };

        let nudge_down = |(ci, ri): (usize, usize)| {
            let mut nudges = vec![];
            if ri == 0 && self.grid[ci][1] == EMPTY && self.grid[ci][2] == EMPTY {
                nudges.push((ci, 2, 2));
            } else if ri == 0 && self.grid[ci][1] == EMPTY && right_col((ci, 2)) {
                nudges.push((ci, 1, 1));
            } else {
                ()
            }
            nudges
        };

        if (vr == 1 || vr == 2) && right_col((vc, 1)) && right_col((vc, 2)) {
            vec![]
        } else if (vr == 2 && self.grid[vc][1] == EMPTY && self.grid[vc][0] == EMPTY) {
            nudge_horiz((vc, 0))
                .into_iter()
                .map(|(a, b, c)| (a, b, c + vr))
                .collect_vec()
        } else if (vr == 1 && self.grid[vc][0] == EMPTY) {
            nudge_horiz((vc, 0))
                .into_iter()
                .map(|(a, b, c)| (a, b, c + vr))
                .collect_vec()
        } else if vr == 0 {
            let correct_col = {
                if ch == A {
                    col_headers[0]
                } else if ch == B {
                    col_headers[1]
                } else if ch == C {
                    col_headers[2]
                } else if ch == D {
                    col_headers[3]
                } else {
                    panic!("panik!!")
                }
            };
            let (lo, hi) = vec![vc, correct_col].into_iter().minmax().into_option().unwrap();
            let path_clear = (lo + 1..hi)
                .filter(|col| *col < self.grid.len())
                .all(|col| self.grid[col][vr] == EMPTY);
            if path_clear {
                let dist = hi - lo;
                nudge_down((correct_col, vr))
                    .into_iter()
                    .map(|(a, b, c)| (a, b, c + dist))
                    .into_iter()
                    .collect_vec()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn one_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        println!("one moves from {:?} on:\n{:?}", from, self);
        let mut moves = vec![];
        let (vc, vr) = from;
        let this = self.grid[vc][vr];
        if vec![
            self.side_spaces,
            self.side_spaces + 2,
            self.side_spaces + 4,
            self.side_spaces + 6,
        ]
        .contains(&vc)
        {
            let ch: u8 = ((vc - self.side_spaces) / 2).try_into().unwrap();
            if (self.grid[vc][vr] == ch && (vr == 2 || vr == 1 && self.grid[vc][2] == ch)) {
                return vec![];
            }
        }

        if vr > 0 && self.grid[vc][vr - 1] == EMPTY && !(vr == 2 && self.grid[vc][0] != EMPTY) {
            let cand = (vc, vr - 1);
            if (vc > 0 && self.grid[vc - 1][0] == EMPTY) {
                moves.push((vc - 1, vr - 1))
            }
            if (vc < self.grid.len() - 1 && self.grid[vc + 1][0] == EMPTY) {
                moves.push((vc + 1, vr - 1))
            }
        }
        if vr < 2
            && self.grid[vc][vr + 1] == EMPTY
            && (self.grid[vc][1] == EMPTY || self.grid[vc][1] == this)
            && (self.grid[vc][2] == EMPTY || self.grid[vc][2] == this)
        {
            moves.push((vc, vr + 1));
        }
        if vc > 0 && self.grid[vc - 1][vr] == EMPTY {
            moves.push((vc - 1, vr))
        }
        if vc < self.grid.len() - 1 && self.grid[vc + 1][vr] == EMPTY {
            moves.push((vc + 1, vr))
        }
        moves.retain(|(endrow, endcol)| {
            self.grid[*endrow][*endcol] == EMPTY && (*endcol == 2 || self.grid[*endrow][*endcol + 1] != EMPTY)
        });
        moves
    }

    // pub fn moves(&self, from: (usize, usize)) {
    //     // let mut moves = vec![];
    //     // let (v)
    // }

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
        visiting: &mut HashMap<Vec<[u8; 3]>, usize>,
        depth: usize,
        cost: usize,
    ) -> Option<usize> {
        if depth == 3 || depth == 4 {
            // println!("visiting at depth {} with cost {} :\n{:?}", depth, cost, self);
        }
        // if memo.contains_key(&self.grid) {
        // return *memo.get(&self.grid).unwrap();
        // }
        if self.solved() {
            // println!("solved board with cost {}", cost);
            return Some(cost);
        }

        // let starts = vec![];
        let mut best: Option<usize> = None;
        for c in 0..self.grid.len() {
            for r in 0..3 {
                if self.grid[c][r] < EMPTY {
                    let valid_moves = self.moves((c, r));
                    if depth < 2 {
                        // println!("valid moves from {:?}: {:?}", (c, r), valid_moves)
                    }
                    for (dc, dr, dist) in valid_moves {
                        let applied = Soln1::mov(self, (c, r), (dc, dr));
                        let mov_cost = Self::cost_one(applied.grid[dc][dr]) * dist;
                        // println!(
                        //     "applied move {:?}=>{:?} (cost = {}):\n{:?}",
                        //     (c, r),
                        //     (dc, dr),
                        //     mov_cost,
                        //     applied
                        // );
                        let new_cost = mov_cost + cost;
                        if best.is_some() && new_cost > best.unwrap() {
                            continue;
                        }
                        if !visiting.contains_key(&applied.grid) || *visiting.get(&applied.grid).unwrap() > new_cost {
                            // let mut new_visiting = visiting.clone();
                            // new_visiting.insert(applied.grid.clone());
                            visiting.insert(applied.grid.clone(), new_cost);
                            // println!("recursing on grid:\n{:?}\nseen:{:?}", applied, memo);
                            let can_solve = applied.recurs(memo, visiting, depth + 1, new_cost);

                            // memo.insert(applied.grid.clone(), can_solve);
                            if let Some(solve_cost) = can_solve {
                                let new_cost = solve_cost;
                                // let new_cost = Self::cost_one(applied.grid[dc][dr]) * dist + solve_cost;
                                if best.is_none() {
                                    // println!("setting best to {} for solve", new_cost);
                                    best = Some(new_cost);
                                } else {
                                    // println!("setting best to {} for solve", best.unwrap().min(new_cost));

                                    best = Some(best.unwrap().min(new_cost))
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("got best: {:?}", best);
        // memo.insert(self.grid.clone(), best);
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
    fn test_one_moves() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        let soln = Soln1::new(parsed, 2);
        println!("board:\n{:?}", soln);

        let mut one_move_boards = vec![];
        for c in 0..soln.grid.len() {
            for r in 0..3 {
                if soln.grid[c][r] < EMPTY {
                    let moves = Soln1::one_moves(&soln, (c, r));
                    for (dc, dr) in moves {
                        let after_move = soln.mov((c, r), (dc, dr));
                        println!("after move:\n{:?}", after_move);
                        one_move_boards.push(after_move);
                    }
                }
            }
        }

        println!("2 moves:");
        for soln in one_move_boards {
            println!("board:\n{:?}", soln);
            for c in 0..soln.grid.len() {
                for r in 0..3 {
                    if soln.grid[c][r] < EMPTY {
                        let moves = Soln1::one_moves(&soln, (c, r));
                        for (dc, dr) in moves {
                            let after_move = soln.mov((c, r), (dc, dr));
                            println!("after move:\n{:?}", after_move);
                            // one_move_boards.push(after_move);
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn test_moves() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        let mut soln = Soln1::new(parsed, 2);
        println!("board:\n{:?}", soln);

        for c in 0..soln.grid.len() {
            for r in 0..3 {
                if soln.grid[c][r] < EMPTY {
                    let moves = soln.moves((c, r));
                    for (dc, dr, dist) in moves {
                        let after_move = soln.mov((c, r), (dc, dr));
                        println!("after {}dist move:\n{:?}", dist, after_move);
                    }
                }
            }
        }

        soln.grid[8][1] = EMPTY;
        soln.grid[10][0] = D;
        println!("board:\n{:?}", soln);

        for c in 0..soln.grid.len() {
            for r in 0..3 {
                if soln.grid[c][r] < EMPTY {
                    let moves = soln.moves((c, r));
                    for (dc, dr, dist) in moves {
                        let after_move = soln.mov((c, r), (dc, dr));
                        println!("after {}dist move:\n{:?}", dist, after_move);
                    }
                }
            }
        }
    }
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
        let mut visiting = HashMap::new();
        let solv = soln.recurs(&mut seen, &mut visiting, 0, 0);
        assert_eq!(solv, Some(12521));
    }
    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
    }
}
