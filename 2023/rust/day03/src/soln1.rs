use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let n = input.len();
        let m = input[0].len();
        let mut special_idxs = vec![];
        for i in 0..n {
            for j in 0..m {
                if input[i][j] != '.' && !input[i][j].is_numeric() {
                    special_idxs.push((i, j))
                }
            }
        }

        let mut nums = vec![];
        let mut seen = vec![vec![false; m]; n];
        for (sx, sy) in special_idxs {
            let mut nbrs = util::grid::nbrs8(sx, sy, m, n);
            nbrs = nbrs
                .into_iter()
                .filter(|(x, y)| input[*x][*y].is_digit(10))
                .collect_vec();
            let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
            for (nx, ny) in nbrs.iter() {
                rows.entry(*nx).or_insert_with(Vec::new).push(*ny);
            }

            for (&rn, cols) in rows.iter() {
                for &j in cols {
                    if seen[rn][j] {
                        continue;
                    }
                    let mut before = (j - 1).max(0);
                    let mut before_digits = vec![];
                    while input[rn][before].is_numeric() && !seen[rn][before] {
                        before_digits.push(input[rn][before]);
                        seen[rn][before] = true;
                        if before > 0 {
                            before -= 1;
                        } else {
                            break;
                        }
                    }
                    before_digits.reverse();

                    let mut after = (j + 1).min(m - 1);
                    let mut after_digits = vec![];
                    while after < m && input[rn][after].is_numeric() && !seen[rn][after] {
                        after_digits.push(input[rn][after]);
                        seen[rn][after] = true;
                        after += 1;
                    }

                    before_digits.push(input[rn][j]);
                    before_digits.extend(after_digits);
                    let num: usize = before_digits.iter().collect::<String>().parse().unwrap();
                    nums.push(num);
                }
            }
        }

        nums.iter().sum()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let n = input.len();
        let m = input[0].len();
        let mut special_idxs = vec![];
        for i in 0..n {
            for j in 0..m {
                if input[i][j] != '.' && !input[i][j].is_numeric() {
                    special_idxs.push((i, j))
                }
            }
        }

        let mut gear_ratio = 0;
        let mut seen = vec![vec![false; m]; n];
        for (sx, sy) in special_idxs {
            let mut nums = vec![];
            let mut nbrs = util::grid::nbrs8(sx, sy, m, n);
            nbrs = nbrs
                .into_iter()
                .filter(|(x, y)| input[*x][*y].is_digit(10))
                .collect_vec();
            let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
            for (nx, ny) in nbrs.iter() {
                rows.entry(*nx).or_insert_with(Vec::new).push(*ny);
            }

            for (&rn, cols) in rows.iter() {
                for &j in cols {
                    if seen[rn][j] {
                        continue;
                    }
                    let mut before = (j - 1).max(0);
                    let mut before_digits = vec![];
                    while input[rn][before].is_numeric() && !seen[rn][before] {
                        before_digits.push(input[rn][before]);
                        seen[rn][before] = true;
                        if before > 0 {
                            before -= 1;
                        } else {
                            break;
                        }
                    }
                    before_digits.reverse();

                    let mut after = (j + 1).min(m - 1);
                    let mut after_digits = vec![];
                    while after < m && input[rn][after].is_numeric() && !seen[rn][after] {
                        after_digits.push(input[rn][after]);
                        seen[rn][after] = true;
                        after += 1;
                    }

                    before_digits.push(input[rn][j]);
                    before_digits.extend(after_digits);
                    let num: usize = before_digits.iter().collect::<String>().parse().unwrap();
                    nums.push(num);
                }
            }
            if input[sx][sy] == '*' && nums.len() == 2 {
                gear_ratio += nums[0] * nums[1]
            }
        }

        gear_ratio
    }
}
