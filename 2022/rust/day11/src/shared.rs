use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

pub type Input = Vec<Monke>;
pub type Output = usize;

pub struct Monke {
    pub items: VecDeque<usize>,
    pub op: Operation,
    pub divisor: usize,
    pub throw_true: usize,
    pub throw_false: usize,
}

pub enum Operation {
    Add(Term),
    Mult(Term),
}

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
    let monkes = vec![
        Monke {
            items: vec![79, 78].into(),
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
