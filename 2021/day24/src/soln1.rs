use itertools::Itertools;

use crate::alu::adt::Instr::*;
use crate::alu::adt::Operand::*;
use crate::alu::adt::Register::{W, X, Y, Z};
use crate::alu::adt::*;
use std::fmt::format;
use std::str::FromStr;

pub struct Eqn {}
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let instrs = Instrs::from_str(input).unwrap();
        let instrs = Instrs(instrs.0.into_iter().rev().collect_vec());
        todo!()
    }

    pub fn compile_cpp(instrs: Instrs) -> String {
        let template_head = "
#include <iostream>
#include <string>
using namespace std;

int main() {
  long W = 0;
  long X = 0;
  long Y = 0;
  long Z = 0;

  int inputs[14] = {};
";
        let template_tail = "
  
  cout << \"Finished\" << endl;
  cout << \"vars=\" << W << \",\" << \",\" << X << \",\" << Y << \",\" << Z << endl;
  return Z == 0;
}";

        let mut prog = String::from(template_head);
        let mut input_num = 0;
        for instr in instrs.0 {
            match instr {
                Inp { dst } => {
                    prog.push_str(&format!("\tcin >> {};\n", dst));
                    input_num += 1;
                }
                Add { dst, operand } => prog.push_str(&format!("\t{} = {} + {};\n", dst, dst, operand)),
                Mul { dst, operand } => prog.push_str(&format!("\t{} = {} * {};\n", dst, dst, operand)),
                Div { dst, operand } => prog.push_str(&format!("\t{} = {} / {};\n", dst, dst, operand)),
                Mod { dst, operand } => prog.push_str(&format!("\t{} = {} % {};\n", dst, dst, operand)),
                Eql { dst, operand } => prog.push_str(&format!("\t{} = {} == {};\n", dst, dst, operand)),
            }
        }
        prog.push_str(template_tail);

        prog

        // let out = match instr {
        //     Inp { dst } => format!(
        //         "asm!(\"mov {{0}}, {{number}}\", inout(reg) {}, number = const {}",
        //         dst, inp
        //     ),
        //     Add { dst, operand } => format!("asm!(\"add {{0}}, {{1}}\", inout(reg) {}, inout(reg) {}", dst, operand),
        //     Mul { dst, operand } => format!("asm!(\"add {{0}}, {{1}}\", inout(reg) {}, inout(reg) {}", dst, operand),
        //     Div { dst, operand } => format!("asm!(\"add {{0}}, {{1}}\", inout(reg) {}, inout(reg) {}", dst, operand),
        //     Mod { dst, operand } => format!("asm!(\"add {{0}}, {{1}}\", inout(reg) {}, inout(reg) {}", dst, operand),
        //     Eql { dst, operand } => format!("asm!(\"add {{0}}, {{1}}\", inout(reg) {}, inout(reg) {}", dst, operand),
        // };

        // println!("w = {}", w);
    }
}

#[cfg(test)]
mod tests {
    use super::Soln1;
    use itertools::Itertools;

    // #[test]
    // fn test_part1() {
    // let sample: &str = include_str!("../inputs/sample.txt");
    // let part1 = Soln1::part1(sample);
    // }
}
