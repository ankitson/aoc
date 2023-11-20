use std::error::Error;

use itertools::{Itertools, PeekingNext};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};

use crate::shared::{parse, Input, Output};
use nom::bytes::complete::{tag, take_while1};
use nom::{branch::alt, combinator::map, *};
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
            (left @ Token::List(_), right @ Token::Literal(_)) => {
                (left.clone()).cmp(&Token::List(vec![*right.clone()]))
            }
            (left @ Token::Literal(_), right @ Token::List(_)) => {
                (&Token::List(vec![*left.clone()])).cmp(right.clone())
            }
            (left @ Token::List(_), right @ Token::List(_)) => left.clone().cmp(right.clone()),

            (Token::Uneval(_), _) => unimplemented!(),
            (_, Token::Uneval(_)) => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug)]
struct TreeString {
    contents: String,
    pos: Vec<usize>,
}

impl TreeString {
    fn tok(packet: &str) -> IResult<&str, String> {
        alt((nom::character::complete::digit1, tag("["), tag("]"), tag(",")))(packet)
            .map(|res_str| (res_str.0, res_str.1.to_string()))
    }
    fn new(contents: &str) -> TreeString {
        TreeString { contents: contents.to_string(), pos: vec![] }
    }
}

impl Iterator for TreeString {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = TreeString::tok(&self.contents);
            // println!("tok: {x:?}");
            let (rem, tok1) = TreeString::tok(&self.contents).unwrap();
            if rem.is_empty() {
                return None;
            }
            self.contents = rem.to_string();
            let mut in_list = false;
            let mut prev = None;
            match tok1.as_str() {
                "[" => {
                    self.pos.push(0);
                    prev = Some("[");
                    in_list = true;
                }
                "]" => {
                    if in_list {}
                    self.pos.pop();
                }
                "," => {
                    let pos = self.pos.pop().unwrap();
                    let idx = pos + 1;
                    self.pos.push(idx);
                }
                num => {
                    let numv = num.parse::<usize>().unwrap();
                    return Some((numv, self.pos.len(), *self.pos.last().unwrap()));
                }
            }
        }
    }
}

impl Soln1 {
    fn parse_into_pairs(raw_input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_list1(tag("\n\n"), separated_pair(take_while1(|s| s != '\n'), tag("\n"), take_while1(|s| s != '\n')))(
            raw_input,
        )
    }

    fn parse_tree_string(str: &str) -> Result<(Option<Token>, &str), Box<dyn Error>> {
        println!("Parsing: {str}");
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

    pub fn part1(raw_input: &str) -> Output {
        let (rem, mut input) = Self::parse_into_pairs(raw_input).unwrap();
        println!("nom parsed:\n{input:?}");
        let newinp = input.iter_mut().map(|(a, b)| (a.to_string(), b.to_string())).collect_vec();
        Self::part1_core(&newinp).expect("fail core")
    }

    pub fn part1_core(input: &Input) -> Result<Output, Box<dyn Error>> {
        println!("core input: {input:?}");
        for (left, right) in input {
            let ltree = Self::parse_tree_string(left);
            let rtree = Self::parse_tree_string(right);
        }
        todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
