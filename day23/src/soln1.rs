use crate::shared::parse;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write,
    iter::{self, repeat},
};
use util;

#[derive(Clone, PartialEq, Eq)]
pub struct Soln1<const N: usize> {
    pub grid: Vec<[u8; N]>,
    pub side_spaces: usize,
}

impl<const N: usize> std::fmt::Debug for Soln1<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut transposed: Vec<Vec<u8>> = vec![vec![0; self.grid.len()]; N];
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
                f.write_char(colchar(char)).expect("panik!!")
            }
            f.write_char('\n').expect("AHHH");
        }
        Ok(())
    }
}

const A: u8 = 0;
const B: u8 = 1;
const C: u8 = 2;
const D: u8 = 3;
const EMPTY: u8 = 4;
const INVALID: u8 = 5;

impl<const N: usize> Soln1<N> {
    pub fn part1(input: &str) -> Option<usize> {
        let parsed = parse::<3>(input);
        let soln = Soln1::<3>::new(parsed, 2);
        // println!("sample grid:\n{:?}", soln);
        // println!("solved?: {}", soln.solved());
        let mut seen = HashMap::new();
        let mut visiting = HashMap::new();
        soln.recurs(&mut seen, &mut visiting, 0, 0)
    }

    pub fn part1_2(input: &str) -> Option<usize> {
        let parsed = parse::<3>(input);
        let soln = Soln1::<3>::new(parsed, 2);
        let mut visiting = HashMap::new();
        soln.recurs2(0, 0, &mut visiting)
    }

    pub fn empty(side_spaces: usize) -> Soln1<N> {
        let hall = || -> [u8; N] {
            let mut arr = [INVALID; N];
            arr[0] = EMPTY;
            arr
        };
        let tunnel = || -> [u8; N] { [EMPTY; N] };

        let mut grid: Vec<[u8; N]> = vec![];
        repeat(hall).take(side_spaces).for_each(|s| grid.push(s()));
        grid.extend(vec![tunnel(), hall(), tunnel(), hall(), tunnel(), hall(), tunnel()].into_iter());
        repeat(hall).take(side_spaces).for_each(|s| grid.push(s()));
        Soln1 { grid, side_spaces }
    }

    pub fn new(cols: [[u8; N]; 4], side_spaces: usize) -> Soln1<N> {
        let mut empty = Soln1::empty(side_spaces);
        for col in [side_spaces, side_spaces + 2, side_spaces + 4, side_spaces + 6] {
            let colnum = (col - side_spaces) / 2;

            empty.grid[col] = cols[colnum];
            // empty.grid[col][0] = EMPTY;
            // for (i, setcol) in cols.iter().enumerate() {
            //     println!(
            //         "grid len: {} i: {} col: {} setcol: {:?}",
            //         empty.grid.len(),
            //         i,
            //         col,
            //         setcol
            //     );
            //     empty.grid[col] = *setcol;
            // }
        }
        empty
    }

    pub fn mov(&self, movfrom: (usize, usize), movto: (usize, usize)) -> Soln1<N> {
        let mut new_board = Soln1 { grid: self.grid.clone(), side_spaces: self.side_spaces };
        let (a, b) = movfrom;
        let (c, d) = movto;
        let mover = new_board.grid[a][b];
        new_board.grid[a][b] = EMPTY;
        new_board.grid[c][d] = mover;
        new_board
    }

    fn occ(&self, ci: usize, ri: usize) -> bool {
        self.grid[ci][ri] != EMPTY
    }
    fn tunnels(&self) -> Vec<usize> {
        let side_spaces = self.side_spaces;
        vec![side_spaces, side_spaces + 2, side_spaces + 4, side_spaces + 6]
    }
    fn correct_pos(&self, ci: usize, ri: usize) -> bool {
        let elem = self.grid[ci][ri];
        if elem == EMPTY {
            return true;
        }
        if elem == INVALID {
            panic!("ahhhhhhhhh!!!!!!!!")
        }
        let tunnel_idx = self.tunnels().iter().map(|ti| ((ti - self.side_spaces) / 2)).collect_vec();
        let is_correct = ci == self.side_spaces + 2 * tunnel_idx[elem as usize];
        is_correct
    }

    fn dst_tunnel_idx(&self, elem: u8) -> usize {
        if elem == EMPTY || elem == INVALID {
            panic!("panic!!!")
        }
        self.tunnels()[elem as usize]
    }

    //(srcol,srcrow,dstcol,dstrow,length of path)
    pub fn moves2(&self, print: bool) -> impl Iterator<Item = (usize, usize, usize, usize, usize)> + '_ {
        let sides = |col_idx| {
            let mut sides_arr = vec![];
            if col_idx > 0 {
                sides_arr.extend(
                    (0..col_idx).rev().filter(|ci| !self.tunnels().contains(ci)).take_while(|ci| !self.occ(*ci, 0)),
                )
            }
            if col_idx < self.grid.len() {
                sides_arr.extend(
                    (col_idx + 1..self.grid.len())
                        .filter(|ci| !self.tunnels().contains(ci))
                        .take_while(|ci| !self.occ(*ci, 0)),
                )
            }
            sides_arr
        };

        let tunnels = self.tunnels();
        let tunnels_with_up_moves = tunnels
            .into_iter()
            .map(|ci| {
                (
                    ci,
                    (0..N).into_iter().find(|ri| {
                        if print {
                            println!("ci, ri = {} {} col: {:?}", ci, ri, self.grid[ci]);
                        }
                        self.grid[ci][*ri] < EMPTY
                            && (0..*ri).into_iter().rev().all(|ur| self.grid[ci][ur] == EMPTY)
                            && !(0..N).into_iter().all(|r| self.correct_pos(ci, r))
                    }),
                )
            })
            .filter(|(ci, mayberi)| mayberi.is_some())
            .map(|(ci, mayberi)| (ci, mayberi.unwrap()));
        let up_moves = tunnels_with_up_moves.flat_map(move |(ci, bad_row)| {
            sides(ci)
                .iter()
                .map(|dest_col| (ci, bad_row, *dest_col, 0usize, (bad_row - 0) + util::abs_diff(ci, *dest_col)))
                .collect_vec()
        });

        let mut down_moves = vec![];
        let hallway_pieces = (0..self.grid.len()).filter(|ci| self.grid[*ci][0] < EMPTY);
        for ci in hallway_pieces {
            let dst_col_idx = self.dst_tunnel_idx(self.grid[ci][0]);
            let wrong_piece_in_tunnel = (1..N).find(|ri| !self.correct_pos(dst_col_idx, *ri));
            if wrong_piece_in_tunnel.is_none() {
                // println!("looking for dst row index in ");
                let dst_row_idx = (1..N)
                    .rev()
                    .find(|ri| {
                        // println!("ci: {} dci: {}", ci, dst_col_idx);
                        let (lo, hi) = vec![0, *ri].into_iter().minmax().into_option().unwrap();
                        (lo..hi + 1).all(|r| self.grid[dst_col_idx][r] == EMPTY)
                        //self.grid[dst_col_idx][*ri] == EMPTY
                    })
                    .unwrap();
                let (lo, hi) = vec![ci, dst_col_idx].into_iter().minmax().into_option().unwrap();
                if (lo..hi + 1).all(|c| c == ci || self.grid[c][0] == EMPTY) {
                    down_moves.push((
                        ci,
                        0usize,
                        dst_col_idx,
                        dst_row_idx,
                        util::abs_diff(dst_col_idx, ci) + util::abs_diff(dst_row_idx, 0),
                    ))
                }
            }
        }

        down_moves.extend(up_moves);
        down_moves.into_iter()

        // let mut iter = iter::empty::<(usize, usize, usize)>();
        // for col_idx in tunnels {
        //     if let Some(bad_row) = (0..N).rev().filter(|ri| !self.correct_pos(col_idx, *ri)).next() {
        //         let left_right = sides(col_idx, 0)
        //             .into_iter()
        //             .map(|dest_col| (dest_col, 0usize, bad_row + col_idx - dest_col));
        //         iter = iter.chain(left_right);
        //     }
        // }
        // self.tunnels()
        //     .into_iter()
        //     .map(|col_idx| {
        //         if let Some(bad_row) = (0..N).rev().filter(|ri| !self.correct_pos(col_idx, *ri)).next() {
        //             let x = sides(col_idx, 0)
        //                 .into_iter()
        //                 .map(|dest_col| (dest_col, 0usize, bad_row + col_idx - dest_col));
        //             x
        //         } else {
        //             iter::empty::<(usize, usize, usize)>().map(|x| x).into_iter()
        //         }
        //     })
        //     .flatten()
    }

    pub fn moves(&self, from: (usize, usize)) -> Vec<(usize, usize, usize)> {
        // let mut moves = vec![]; //(dest,dist)
        let (vc, vr) = from;
        let ch = self.grid[vc][vr];
        let col_headers = vec![self.side_spaces, self.side_spaces + 2, self.side_spaces + 4, self.side_spaces + 6];

        let nudge_horiz = |(ci, ri): (usize, usize)| {
            let col_headers = vec![self.side_spaces, self.side_spaces + 2, self.side_spaces + 4, self.side_spaces + 6];
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
            nudge_horiz((vc, 0)).into_iter().map(|(a, b, c)| (a, b, c + vr)).collect_vec()
        } else if (vr == 1 && self.grid[vc][0] == EMPTY) {
            nudge_horiz((vc, 0)).into_iter().map(|(a, b, c)| (a, b, c + vr)).collect_vec()
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
            let path_clear = (lo + 1..hi).filter(|col| *col < self.grid.len()).all(|col| self.grid[col][vr] == EMPTY);
            if path_clear {
                let dist = hi - lo;
                nudge_down((correct_col, vr)).into_iter().map(|(a, b, c)| (a, b, c + dist)).into_iter().collect_vec()
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
        if vec![self.side_spaces, self.side_spaces + 2, self.side_spaces + 4, self.side_spaces + 6].contains(&vc) {
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
        for col in [self.side_spaces, self.side_spaces + 2, self.side_spaces + 4, self.side_spaces + 6] {
            let ch: u8 = ((col - self.side_spaces) / 2).try_into().unwrap();
            valid &= self.grid[col][0] == EMPTY;
            valid &= self.grid[col][1] == ch;
            valid &= self.grid[col][2] == ch;
        }
        valid
    }

    fn recurs2(&self, cost: usize, depth: usize, visiting: &mut HashMap<Vec<[u8; N]>, usize>) -> Option<usize> {
        if self.solved() {
            // println!("solved with cost: {}", cost);
            return Some(cost);
        }

        let mut best: Option<usize> = None;

        if depth < 16 {
            // println!("board (d={}):\n{:?}", depth, self);
            //println!("board:\n{:?}", self);
            // println!("moves:\n{:?}", self.moves2(false).collect_vec());
        }
        for mov in self.moves2(false) {
            // println!("considering move {:?}", mov);
            let (src_col, src_row, dst_col, dst_row, path_len) = mov;
            let applied = Soln1::mov(self, (src_col, src_row), (dst_col, dst_row));
            // println!("next:\n{:?}", applied);
            let cost_after = Self::cost_one(applied.grid[dst_col][dst_row]) * path_len + cost;
            if !visiting.contains_key(&applied.grid) || *visiting.get(&applied.grid).unwrap() > cost_after {
                visiting.insert(applied.grid.clone(), cost_after);

                let can_solve = applied.recurs2(cost_after, depth + 1, visiting);

                // println!("got soln: {:?}", can_solve);
                // memo.insert(applied.grid.clone(), can_solve);
                if let Some(solve_cost) = can_solve {
                    let new_cost = solve_cost;
                    if best.is_none() {
                        best = Some(new_cost);
                    } else {
                        best = Some(best.unwrap().min(new_cost))
                    }
                }
            }
        }
        best
    }

    fn recurs(
        &self,
        memo: &mut HashMap<Vec<[u8; N]>, Option<usize>>,
        visiting: &mut HashMap<Vec<[u8; N]>, usize>,
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

    pub fn part2(input: &str) -> Option<usize> {
        let mut lines = input.lines().collect_vec();
        lines.insert(3, "  #D#C#B#A#");
        lines.insert(4, "  #D#B#A#C#");
        let expanded = lines.join("\n");
        let parsed = parse::<5>(&expanded);
        let soln = Soln1::<5>::new(parsed, 2);
        let mut visiting = HashMap::new();
        soln.recurs2(0, 0, &mut visiting)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;

    use super::Soln1;
    use crate::{
        shared::{parse, parse2},
        soln1::A,
        soln1::C,
        soln1::D,
        soln1::{B, EMPTY},
    };

    #[test]
    fn test_moves() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let parsed = parse::<3>(sample);
        let mut soln = Soln1::<3>::new(parsed, 2);
        println!("board:\n{:?}", soln);

        soln.grid[8][1] = EMPTY;
        soln.grid[10][0] = D;
        println!("board:\n{:?}", soln);

        let moves = soln.moves2(false).collect_vec();

        println!("moves: {:?}", moves);
        for (sc, sr, dc, dr, dist) in moves {
            let after_move = soln.mov((sc, sr), (dc, dr));
            println!("after {}dist move:\n{:?}", dist, after_move);
        }
    }
    #[test]
    fn test_paths() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let parsed = parse::<3>(sample);
        let soln = Soln1::<3>::empty(2);
        println!("empty:\n{:?}", soln);
        println!("solved?: {}", soln.solved());

        let mut solved = Soln1::<3>::empty(0);
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

        // let soln = Soln1::new(parsed, 2);
        // println!("sample grid:\n{:?}", soln);
        // println!("solved?: {}", soln.solved());
        // let mut seen = HashMap::new();
        // let mut visiting = HashMap::new();
        // let solv = soln.recurs(&mut seen, &mut visiting, 0, 0);
        // assert_eq!(solv, Some(12521));
    }

    #[test]
    fn test_compare_moves() {
        let sample: &str = include_str!("../inputs/sample.txt");

        let grid1 = parse::<3>(sample);
        let mut soln1 = Soln1::new(grid1, 2);
        let mut soln2 = Soln1::new(grid1, 2);
        for i in 0..5 {
            let moves1 = |soln: &Soln1<3>| {
                let mut moves = vec![];
                for c in 0..soln.grid.len() {
                    for r in 0..3 {
                        if soln.grid[c][r] < EMPTY {
                            let gen1 = soln1.moves((c, r));
                            moves.extend(gen1.into_iter().map(|(a, b, d)| (c, r, a, b, d)));
                        }
                    }
                }
                moves
            };
            let moves2 = |soln: &Soln1<3>| soln.moves2(false).collect_vec();

            let mut _moves1 = moves1(&soln1);
            let mut _moves2 = moves2(&soln2);
            _moves1.sort();
            _moves2.sort();
            assert_eq!(_moves1, _moves2);
            use rand::prelude::*;
            let mut rng = rand::thread_rng();
            let mov_idx = rng.gen_range(0.._moves1.len());
            // let mov_idx = rand
            soln1 = soln1.mov((_moves1[mov_idx].0, _moves1[mov_idx].1), (_moves1[mov_idx].2, _moves1[mov_idx].3));
            soln2 = soln2.mov((_moves2[mov_idx].0, _moves2[mov_idx].1), (_moves2[mov_idx].2, _moves2[mov_idx].3));
            assert_eq!(soln1, soln2);
            println!("board:\n{:?}", soln1);
        }

        // println!("moves 1:\n{:?}", moves1(&soln1));
        // println!("moves 2:\n{:?}", moves2(soln2));
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        println!("sample: {:?}", Soln1::<3>::part1_2(sample));
    }
}
