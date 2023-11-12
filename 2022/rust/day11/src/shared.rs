use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

use nom::branch::alt;
pub use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::{multispace0, space0, space1};
use nom::combinator::{map, value};
use nom::IResult;

use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, tuple};

pub type Input = Vec<Monke>;
pub type Output = usize;

#[derive(Debug, Clone)]
pub struct Monke {
    pub items: VecDeque<usize>,
    pub op: Operation,
    pub divisor: usize,
    pub throw_true: usize,
    pub throw_false: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Term),
    Mult(Term),
}

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Prev,
    Const(usize),
}

impl Operation {
    pub fn eval(&self, prev: usize) -> usize {
        match self {
            Operation::Add(Term::Prev) => prev + prev,
            Operation::Add(Term::Const(other)) => prev + other,
            Operation::Mult(Term::Prev) => prev * prev,
            Operation::Mult(Term::Const(other)) => prev * other,
        }
    }
}

pub fn parse(input: &str) -> Input {
    fn num(input: &str) -> IResult<&str, usize> {
        map(nom::character::complete::u32, |n| n as usize)(input)
    }
    fn header(input: &str) -> IResult<&str, usize> {
        preceded(tag("Monkey "), num)(input)
    }
    fn starting(input: &str) -> IResult<&str, VecDeque<usize>> {
        preceded(space1, preceded(tag("Starting items: "), map(separated_list1(tag(","), num), |x| x.into())))(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        let (rem, _) = preceded(space1, tag("Operation: new = old "))(input)?;
        let (rem, op) = alt((tag("*"), tag("+")))(rem)?;
        let (rem, rhs) = preceded(space0, alt((map(num, |x| Term::Const(x)), value(Term::Prev, tag("old")))))(rem)?;
        let op = match (op, rhs) {
            ("+", term) => Operation::Add(rhs),
            ("*", term) => Operation::Mult(rhs),
            _ => unreachable!(),
        };
        Ok((rem, op))
    }
    fn divtest(input: &str) -> IResult<&str, usize> {
        let (rem, _) = preceded(space1, tag("Test: divisible by "))(input)?;
        let (rem, rhs) = num(input)?;
        Ok((rem, rhs))
    }
    fn branch(input: &str) -> IResult<&str, (bool, usize)> {
        let (rem, branch_pred) = alt((
            value(true, preceded(space1, tag("If true: throw to monkey "))),
            value(false, preceded(space1, tag("If false: throw to monkey "))),
        ))(input)?;
        let (rem, rhs) = num(rem)?;
        Ok((rem, (branch_pred, rhs)))
    }
    fn monke(input: &str) -> IResult<&str, Monke> {
        let (rem, monke_num) = header(input)?;
        let (rem, start_items) = starting(rem)?;
        let (rem, op) = operation(rem)?;
        let (rem, divisor) = divtest(rem)?;
        let (rem, (true, tbranch)) = branch(rem)? else { panic!("parse error") };
        let (rem, (false, fbranch)) = branch(rem)? else { panic!("parse error") };
        let monke = Monke { items: start_items, divisor: divisor, op: op, throw_true: tbranch, throw_false: fbranch };
        Ok((rem, monke))
    }
    fn monkes(input: &str) -> IResult<&str, Vec<Monke>> {
        separated_list0(space1, monke)(input)
    }

    let (rem, parsed) = monkes(input).unwrap();
    parsed
}

pub fn _parse(input: &str) -> Input {
    let monkes = vec![
        Monke {
            items: vec![79, 98].into(),
            op: Operation::Mult(Term::Const(19)),
            divisor: 23,
            throw_true: 2,
            throw_false: 3,
        },
        Monke {
            items: vec![54, 65, 75, 74].into(),
            op: Operation::Add(Term::Const(6)),
            divisor: 19,
            throw_true: 2,
            throw_false: 0,
        },
        Monke {
            items: vec![79, 60, 97].into(),
            op: Operation::Mult(Term::Prev),
            divisor: 13,
            throw_true: 1,
            throw_false: 3,
        },
        Monke {
            items: vec![74].into(),
            op: Operation::Add(Term::Const(3)),
            divisor: 17,
            throw_true: 0,
            throw_false: 1,
        },
    ];

    return monkes;
}
