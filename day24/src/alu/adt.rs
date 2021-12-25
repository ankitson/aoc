#[derive(Debug, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Register { W, X, Y, Z, }

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    RegOp(Register),
    Literal(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instr {
    Inp { dst: Register },
    Add { dst: Register, operand: Operand },
    Mul { dst: Register, operand: Operand },
    Div { dst: Register, operand: Operand },
    Mod { dst: Register, operand: Operand },
    Eql { dst: Register, operand: Operand },
}

pub type Instrs = Vec<Instr>;
