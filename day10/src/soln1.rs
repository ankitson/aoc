use itertools::Itertools;
use std::collections::HashMap;

#[path = "shared.rs"]
mod shared;

enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct Soln1 {
    brackets: Vec<char>,
    bracket_matchers: HashMap<char, char>,
    scores: HashMap<char, i32>,
}
/**
 * Compared to the previous commit, this is a more
 * high-level soln similar to a typical Python/Scala soln
 * using "classes" & dictionaries liberally.
 *
 * It runs ~50-100% slower (from ~60us to 90-120us)
 */
impl Soln1 {
    pub fn new() -> Soln1 {
        let brs = [(')', '('), (']', '['), ('}', '{'), ('>', '<')];
        let scores1 = brs.map(|x| x.1).zip([3, 57, 1197, 25137]);
        let scores2 = brs.map(|x| x.0).zip([1, 2, 3, 4]);
        let scores = [scores1, scores2].concat();
        Soln1 {
            brackets: brs.iter().map(|x| x.1).collect_vec(),
            bracket_matchers: HashMap::from_iter(brs),
            scores: HashMap::from_iter(scores), // scores: HashMap::from_iter(scores1.(scores2)),
        }
    }

    fn first_illegal(&self, line: &str) -> Either<usize, Vec<char>> {
        let mut stack: Vec<char> = Vec::new();
        let mut illegal_idx: Option<usize> = None;
        for (idx, char) in line.chars().enumerate() {
            if self.brackets.contains(&char) {
                stack.push(char);
            } else {
                let matching_open = self.bracket_matchers.get(&char).unwrap();
                if *stack.last().unwrap() == *matching_open {
                    stack.pop();
                } else {
                    illegal_idx = Some(idx);
                    break;
                }
            }
        }
        if let Some(bad_idx) = illegal_idx {
            Either::Left(bad_idx)
        } else {
            Either::Right(stack.clone())
        }
    }

    fn score_illegal(bracket: &char) -> usize {
        match bracket {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("illegal char"),
        }
    }

    pub fn part1(&self, input: &str) -> usize {
        let lines = shared::parse(input);
        let mut total_score = 0;
        for line in lines {
            let line_score = match Self::first_illegal(self, line) {
                Either::Left(bad_index) => Self::score_illegal(&line.chars().nth(bad_index).unwrap()),
                _ => 0,
            };
            total_score += line_score;
        }
        total_score
    }

    fn score_completion(&self, completion: Vec<char>) -> i64 {
        let mut score = 0i64;
        let inverted_map: HashMap<char, char> = HashMap::from_iter(self.bracket_matchers.iter().map(|x| (*x.1, *x.0)));
        for char in completion.iter().rev() {
            let br_score = *self.scores.get(inverted_map.get(char).unwrap()).unwrap();
            score = (score * 5) + (i64::from(br_score));
        }
        score
    }

    pub fn part2(&self, input: &str) -> i64 {
        let lines = shared::parse(input);
        let mut scores: Vec<i64> = vec![];
        for line in lines {
            let case = Self::first_illegal(self, line);
            if let Either::Right(stack) = case {
                let completion_score = Self::score_completion(self, stack);
                scores.push(completion_score);
            }
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}
