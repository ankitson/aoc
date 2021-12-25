use std::{cell::Cell, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[rustfmt::skip]
pub enum Register { W, X, Y, Z, }

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Register::W => write!(f, "W"),
            Register::X => write!(f, "X"),
            Register::Y => write!(f, "Y"),
            Register::Z => write!(f, "Z"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    RegOp(Register),
    Literal(isize),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operand::RegOp(reg) => write!(f, "{}", reg),
            Operand::Literal(num) => write!(f, "{}", num),
        }
    }
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
