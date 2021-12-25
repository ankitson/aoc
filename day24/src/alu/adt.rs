use std::{cell::Cell, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[rustfmt::skip]
pub enum Register { W, X, Y, Z, }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    RegOp(Register),
    Literal(isize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instr {
    Inp { dst: Register },
    Add { dst: Register, operand: Operand },
    Mul { dst: Register, operand: Operand },
    Div { dst: Register, operand: Operand },
    Mod { dst: Register, operand: Operand },
    Eql { dst: Register, operand: Operand },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instrs(pub Vec<Instr>);
