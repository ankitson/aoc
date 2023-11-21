use std::cmp::Ordering;

use anyhow::Result;
use itertools::Itertools;

use crate::shared::{self, Input, Output};
pub struct Soln1 {}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    Literal(usize),
    List(Vec<Token<'a>>),
    Uneval(&'a str),
}

impl<'a> PartialOrd for Token<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> Ord for Token<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, &other) {
            (Token::Literal(a), Token::Literal(b)) => a.cmp(&b),
            (left @ Token::List(_), _right @ Token::Literal(v)) => (**left).cmp(&Token::List(vec![Token::Literal(*v)])),
            (_left @ Token::Literal(v), right @ Token::List(_)) => (&Token::List(vec![Token::Literal(*v)])).cmp(*right),
            (_left @ Token::List(lv), _right @ Token::List(rv)) => {
                for item in lv.iter().zip_longest(rv.iter()) {
                    match item {
                        itertools::EitherOrBoth::Both(v1, v2) => {
                            let elem_cmp = (*v1).cmp(v2);
                            if elem_cmp != Ordering::Equal {
                                return elem_cmp;
                            }
                        }
                        itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }
                return Ordering::Equal;
            }

            (Token::Uneval(_), _) => unimplemented!(),
            (_, Token::Uneval(_)) => unimplemented!(),
        }
    }
}

impl Soln1 {
    fn parse_tree_string(str: &str) -> Result<(Option<Token>, &str)> {
        if str.is_empty() {
            return Ok((None, ""));
        }

        if str.chars().next().unwrap() == '[' {
            let mut rem = &str[1..];
            let mut elems: Vec<Token<'_>> = vec![];
            while rem.chars().next().unwrap() != ']' {
                let (tok_maybe, rem_s) = Self::parse_tree_string(rem)?;
                if let Some(tok) = tok_maybe {
                    elems.push(tok);
                }
                rem = rem_s;
            }
            rem = &rem[1..];
            if rem.chars().nth(0) == Some(',') {
                rem = &rem[1..];
            }
            return Ok((Some(Token::List(elems)), rem));
        } else if str.chars().next().unwrap().is_ascii_digit() {
            let end_idx = str.chars().position(|x| !x.is_ascii_digit()).unwrap_or(str.len());
            let (num_str, mut rem) = str.split_at(end_idx);
            let num = num_str.parse::<usize>()?;
            if rem.chars().nth(0) == Some(',') {
                rem = &rem[1..];
            }
            return Ok((Some(Token::Literal(num)), rem));
        } else {
            panic!("unexpected char")
        }
    }

    pub fn part1(raw_input: &'static str) -> Output {
        let input = shared::parse(raw_input).unwrap();
        let ordered = Self::part1_core(&input).expect("part1_core failed");
        ordered.iter().sum()
    }

    pub fn part1_core(input: &Input) -> Result<Vec<usize>> {
        let mut ordered = vec![];
        for (idx, (left, right)) in input.iter().enumerate() {
            let ltree = Self::parse_tree_string(left)?;
            let rtree = Self::parse_tree_string(right)?;
            if ltree < rtree {
                ordered.push(idx + 1)
            }
        }
        Ok(ordered)
    }

    pub fn part2(raw_input: &'static str) -> Output {
        let input = shared::parse(raw_input).unwrap();
        let ans = Self::part2_core(&input).expect("part2_core failed");
        ans.0 * ans.1
    }

    pub fn part2_core(input: &Input) -> Result<(usize, usize)> {
        let packet2 = Token::List(vec![Token::List(vec![Token::Literal(2)])]);
        let packet6 = Token::List(vec![Token::List(vec![Token::Literal(6)])]);
        let mut extended = input
            .iter()
            .flat_map(|v| {
                [Self::parse_tree_string(&v.0).unwrap().0.unwrap(), Self::parse_tree_string(&v.1).unwrap().0.unwrap()]
            })
            .chain([packet2.clone(), packet6.clone()])
            .collect_vec();
        extended.sort();
        let index2 = extended.iter().position(|v| v == &packet2).unwrap() + 1;
        let index6 = extended.iter().position(|v| v == &packet6).unwrap() + 1;
        Ok((index2, index6))
    }
}
