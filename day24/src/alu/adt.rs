use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[rustfmt::skip]
pub enum Register { W, X, Y, Z, Other(String) }

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Register::W => write!(f, "W"),
            Register::X => write!(f, "X"),
            Register::Y => write!(f, "Y"),
            Register::Z => write!(f, "Z"),
            Register::Other(s) => write!(f, "{}", s),
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

use Instr::*;
use Register::{Other, W, X, Y, Z};
impl Instrs {
    pub fn ssa(&self) {}
    // pub fn lifetimes(&self) -> HashMap<Register, Vec<(usize, usize)>> {
    //     let mut lifes = HashMap::from_iter([(W, vec![]), (X, vec![]), (Y, vec![]), (Z, vec![])]);
    //     let mut currstarts: HashMap<Register, usize> = HashMap::from_iter([(W, 0), (X, 0), (Y, 0), (Z, 0)]);

    //     let mut start_new = |reg, inum| {
    //         let curr_start = currstarts.get(reg).unwrap();
    //         lifes.entry(*reg).and_modify(|v| v.push((*curr_start, inum)));
    //         currstarts.entry(*reg).and_modify(|f| *f = inum + 1);
    //     };

    //     for (inum, instr) in self.0.iter().enumerate() {
    //         match instr {
    //             Inp { dst } => start_new(dst, inum),
    //             // Add { dst, operand } if matches!(operand, Operand::Literal(_)) => start_new(dst, inum),
    //             Mul { dst, operand } if matches!(operand, Operand::Literal(0)) => start_new(dst, inum),
    //             // Div { dst, operand } if matches!(operand, Operand::Literal(_)) => start_new(dst, inum),
    //             // Mod { dst, operand } if matches!(operand, Operand::Literal(_)) => start_new(dst, inum),
    //             // Eql { dst, operand } if matches!(operand, Operand::Literal(_)) => start_new(dst, inum),
    //             _ => (),
    //         }
    //     }
    //     lifes
    // }
}
