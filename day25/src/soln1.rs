use core::num;
use std::fmt::{Debug, Write};

use crate::shared::{parse, DOWN, EMPTY, RIGHT};
use itertools::Itertools;
use util;

#[derive(PartialEq, Eq)]
pub struct Soln1<'a> {
    rows: &'a mut Vec<Vec<u8>>,
}

impl<'a> Debug for Soln1<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for char in row {
                let ch = match *char {
                    EMPTY => '.',
                    DOWN => 'v',
                    RIGHT => '>',
                    _ => panic!("illegal"),
                };
                f.write_char(ch);
            }
            f.write_char('\n');
        }
        Ok(())
    }
}

impl<'a> Soln1<'a> {
    pub fn step(&mut self) {
        self.right();
        self.down();
    }

    pub fn stepn(&mut self, n: usize) {
        for i in 0..n {
            self.step()
        }
    }

    pub fn part1(input: &str) -> usize {
        let rows = parse(input);

        let mut soln = Soln1 {
            rows: &mut rows.clone(),
        };
        let mut num_steps = 1;
        let mut last_rows = rows;
        soln.right();
        soln.down();
        while *soln.rows != last_rows {
            last_rows = soln.rows.clone();
            soln.right();
            soln.down();
            num_steps += 1;
        }
        num_steps
    }

    fn right(&mut self) {
        let mut next = vec![];
        for i in 0..self.rows.len() {
            let mut next_row = vec![EMPTY; self.rows[i].len()];
            for j in 0..self.rows[i].len() {
                let nc = (j + 1) % self.rows[i].len();
                if self.rows[i][j] == RIGHT && self.rows[i][nc] == EMPTY {
                    next_row[nc] = self.rows[i][j];
                } else if self.rows[i][j] != EMPTY {
                    next_row[j] = self.rows[i][j];
                }
            }
            next.push(next_row)
        }
        self.rows.clone_from(&next);
    }

    fn down(&mut self) {
        let mut next = vec![vec![EMPTY; self.rows[0].len()]; self.rows.len()];
        for j in 0..self.rows[0].len() {
            // let mut next_col = vec![EMPTY; self.rows.len()];
            for i in 0..self.rows.len() {
                let nr = (i + 1) % self.rows.len();
                if self.rows[i][j] == DOWN && self.rows[nr][j] == EMPTY {
                    next[nr][j] = self.rows[i][j];
                } else if self.rows[i][j] != EMPTY {
                    next[i][j] = self.rows[i][j];
                }
            }
        }
        self.rows.clone_from(&next);
    }

    pub fn part2(input: &str) -> Option<usize> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::shared::parse;

    use super::Soln1;
    use itertools::Itertools;
    use regex::Regex;

    fn parse_steps(steps_str: &str) -> HashMap<usize, String> {
        let mut steps = HashMap::new();
        let mut lines = steps_str.lines();
        let mut step_no = 0;
        loop {
            let line_op = lines.next();
            if line_op.is_none() {
                break;
            }
            let mut line = line_op.unwrap();
            let re = Regex::new(r"^[^\d]+(\d+)").unwrap();
            if line.starts_with("Initial") || line.starts_with("After") {
                step_no = {
                    if line.starts_with("Initial") {
                        0
                    } else {
                        re.captures(line)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap()
                    }
                };
                let mut str = String::new();
                let mut line_op = lines.next();
                while line_op.is_some() && !line_op.unwrap().is_empty() {
                    str.push_str(line_op.unwrap());
                    str.push_str("\n");
                    line_op = lines.next();
                }
                steps.insert(step_no, str);
            }
        }
        steps
    }

    #[test]
    fn test_sample() {
        let steps_str = include_str!("../inputs/steps.txt");
        let sample_steps = parse_steps(steps_str);

        for i in 0..=60 {
            if sample_steps.contains_key(&i) {
                let mut part1 = Soln1 {
                    rows: &mut parse(sample_steps.get(&0).unwrap()),
                };
                part1.stepn(i);
                let mine = format!("{:?}", part1);
                let expected = sample_steps.get(&i).unwrap().trim();
                let matched = mine.trim() == expected.trim();
                if !matched {
                    println!("Step {} mismatched.\nExpected:\n{}\nActual:\n{:?}", i, expected, part1);
                }
            }
        }
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        println!("sample: {}", sample);
        let part1 = Soln1::part1(sample);
        assert_eq!(part1, 58);
    }
}
