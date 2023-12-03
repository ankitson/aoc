use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    ///Sum of all part numbers
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
            let nbrs = util::grid::nbrs8(sx, sy, m, n);
            let nbrs = nbrs.iter().filter(|(x, y)| input[*x][*y].is_digit(10));
            let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
            for (nx, ny) in nbrs {
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

    pub fn part1_fast(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_fast_core(&input)
    }

    pub fn part1_fast_core(input: &Input) -> Output {
        let n = input.len();
        let m = input[0].len();

        let mut partsum = 0;
        for i in 0..n {
            let mut num = 0;
            let mut is_part = false;
            for j in 0..m {
                if input[i][j].is_ascii_digit() {
                    num = 10 * num + (input[i][j].to_digit(10).unwrap() as usize);
                    for ni in i.saturating_sub(1)..=i + 1 {
                        for nj in j.saturating_sub(1)..=j + 1 {
                            if ni < n && nj < m {
                                if !(input[ni][nj] == '.' || input[ni][nj].is_ascii_digit()) {
                                    is_part = true;
                                }
                            }
                        }
                    }
                } else {
                    if is_part {
                        partsum += num;
                    }
                    num = 0;
                    is_part = false;
                }
            }
            if is_part {
                partsum += num;
            }
        }
        partsum
    }

    //Sum of gear ratios
    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_fast(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_fast_core(&input)
    }

    pub fn part2_fast_core(input: &Input) -> Output {
        let n = input.len();
        let m = input[0].len();
        let mut gear_parts: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        for i in 0..n {
            let mut num = 0;
            let mut gears = HashSet::new();
            for j in 0..m + 1 {
                if j < m && input[i][j].is_ascii_digit() {
                    num = 10 * num + (input[i][j].to_digit(10).unwrap() as usize);
                    for ni in i.saturating_sub(1)..=i + 1 {
                        for nj in j.saturating_sub(1)..=j + 1 {
                            if ni < n && nj < m {
                                if input[ni][nj] == '*' {
                                    gears.insert((ni, nj));
                                }
                            }
                        }
                    }
                } else if num > 0 {
                    for gear in &gears {
                        gear_parts
                            .entry(*gear)
                            .and_modify(|v| v.push(num))
                            .or_insert(vec![num]);
                    }
                    num = 0;
                    gears = HashSet::new();
                }
            }
        }

        let mut gearsum = 0;
        for v in gear_parts.values() {
            if v.len() == 2 {
                gearsum += v[0] * v[1];
            }
        }
        gearsum
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
            let nbrs = util::grid::nbrs8(sx, sy, m, n);
            let nbrs = nbrs.iter().filter(|(x, y)| input[*x][*y].is_digit(10));
            let mut rows: HashMap<usize, Vec<usize>> = HashMap::new();
            for (nx, ny) in nbrs {
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
