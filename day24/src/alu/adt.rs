use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq)]
pub struct ALU {
    pub vals: HashMap<Register, isize>,
}

use Instr::{Add, Div, Eql, Inp, Mod, Mul};
use Operand::{Literal, RegOp};
use Register::{W, X, Y, Z};

impl ALU {
    pub fn default() -> Self {
        let vals = HashMap::from_iter(vec![(W, 0), (X, 0), (Y, 0), (Z, 0)]);
        ALU { vals }
    }

    fn get_operand_val(&self, operand: Operand) -> isize {
        match operand {
            RegOp(reg) => *self.vals.get(&reg).unwrap(),
            Literal(i) => i,
        }
    }

    pub fn eval_one(&mut self, instr: Instr, input: Option<isize>) -> () {
        match instr {
            Inp { dst } => {
                self.vals.insert(dst, input.unwrap());
            }
            Add { dst, operand } => {
                let d = self.vals.entry(dst).and_modify(|d| *d += self.get_operand_val(operand));
                // d += self.get_operand_val(operand);
                // let operand_val = self.vals.get_mut(&dst).map(|d| *d += self.get_operand_val(operand));
            }
            Mul { dst, operand } => todo!(),
            Div { dst, operand } => todo!(),
            Mod { dst, operand } => todo!(),
            Eql { dst, operand } => todo!(),
            // _ => panic!("not impl"),
        }
    }
}
