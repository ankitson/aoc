use itertools::Itertools;
use std::{cell::Cell, collections::HashMap};

use crate::alu::adt::Instr::*;
use crate::alu::adt::Operand::*;
use crate::alu::adt::Register::*;
use crate::alu::adt::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ALU {
    pub vals: HashMap<Register, Cell<isize>>,
}

impl ALU {
    pub fn default() -> Self {
        let vals = HashMap::from_iter(vec![
            (W, Cell::new(0)),
            (X, Cell::new(0)),
            (Y, Cell::new(0)),
            (Z, Cell::new(0)),
        ]);
        ALU { vals }
    }

    pub fn reset(&mut self) {
        for (k, v) in &self.vals {
            v.set(0);
        }
    }

    fn get_operand_val(&self, operand: Operand) -> isize {
        match operand {
            RegOp(reg) => self.vals.get(&reg).unwrap().get(),
            Literal(i) => i,
        }
    }

    pub fn eval_one(&mut self, instr: Instr, input: Option<isize>) -> () {
        match instr {
            Inp { dst } => {
                self.vals.entry(dst).and_modify(|d| d.set(input.unwrap()));
                ()
            }
            Add { dst, operand } => {
                self.vals
                    .get(&dst)
                    .unwrap()
                    .set(self.vals.get(&dst).unwrap().get() + self.get_operand_val(operand));
            }
            Mul { dst, operand } => {
                self.vals
                    .get(&dst)
                    .unwrap()
                    .set(self.vals.get(&dst).unwrap().get() * self.get_operand_val(operand));
            }
            Div { dst, operand } => {
                self.vals
                    .get(&dst)
                    .unwrap()
                    .set(self.vals.get(&dst).unwrap().get() / self.get_operand_val(operand));
            }
            Mod { dst, operand } => {
                self.vals
                    .get(&dst)
                    .unwrap()
                    .set(self.vals.get(&dst).unwrap().get() % self.get_operand_val(operand));
            }
            Eql { dst, operand } => {
                let bool_lit = |b: bool| if b { 1 } else { 0 };
                self.vals.get(&dst).unwrap().set(bool_lit(
                    self.vals.get(&dst).unwrap().get() == self.get_operand_val(operand),
                ));
            } // _ => panic!("not impl"),
        }
    }

    pub fn eval(&mut self, instrs: Instrs, inputs: Vec<isize>) {
        let mut input_num = 0;
        for instr in instrs.0 {
            match instr {
                Inp { .. } => {
                    if input_num >= inputs.len() {
                        panic!("Not enough inputs!");
                    }
                    self.eval_one(instr, Some(inputs[input_num]));
                    input_num += 1;
                }
                _ => self.eval_one(instr, None),
            }
        }
    }

    pub fn state(&self) -> Vec<isize> {
        let registers = vec![W, X, Y, Z];
        registers
            .into_iter()
            .map(|reg| self.vals.get(&reg).unwrap().get())
            .collect_vec()
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    fn eval_one(alu: &mut ALU, instr: Instr, input: Option<isize>, expected: Vec<(Register, isize)>) {
        alu.eval_one(instr, input);
        for (reg, expect_val) in expected {
            assert_eq!(alu.vals.get(&reg).unwrap().get(), expect_val);
        }
    }

    #[test]
    fn test_eval() {
        let mut alu = ALU::default();
        eval_one( &mut alu, Add { dst: X, operand: Literal(1), }, None, vec![(X, 1)] );
        alu.reset();

        eval_one( &mut alu, Add { dst: X, operand: Literal(2), }, None, vec![(X, 2)] );
        eval_one( &mut alu, Add { dst: Y, operand: Literal(99), }, None, vec![(Y, 99)] );
        eval_one( &mut alu, Mul { dst: Y, operand: RegOp(X), }, None, vec![(Y, 2*99)] );
        eval_one( &mut alu, Mod { dst: Y, operand: Literal(5), }, None, vec![(Y, 3)] );
        eval_one( &mut alu, Div { dst: Y, operand: Literal(7), }, None, vec![(Y, 0)] );
        
        eval_one( &mut alu, Inp { dst: Z, }, Some(55), vec![(Z, 55)] );
        eval_one( &mut alu, Mul { dst: Z, operand: Literal(0), }, None, vec![(Z, 0)] );
        eval_one( &mut alu, Inp { dst: Z, }, Some(55), vec![(Z, 55)] );
        eval_one( &mut alu, Div { dst: Z, operand: RegOp(Z), }, None, vec![(Z, 1)] );
        eval_one( &mut alu, Eql { dst: Z, operand: Literal(1), }, None, vec![(Z, 1)] );
        eval_one( &mut alu, Add { dst: Y, operand: Literal(1), }, None, vec![(Y, 1)] );
        eval_one( &mut alu, Eql { dst: Z, operand: RegOp(Y), }, None, vec![(Z,1), (Y, 1)] );

        assert_eq!(alu.state(), vec![0,2,1,1]);
    }
}
