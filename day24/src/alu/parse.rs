use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

use super::adt::Register::{W, X, Y, Z};
use super::adt::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[rustfmt::skip]
pub struct ParseErr { 
    pub input: String, 
    pub at: usize, 
    pub expect: String, 
    pub inner: Option<Result<Box<ParseErr>, ParseIntError>> 
}

impl FromStr for Register {
    type Err = ParseErr;

    fn from_str(input: &str) -> Result<Register, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "W" => Ok(W),
            "X" => Ok(X),
            "Y" => Ok(Y),
            "Z" => Ok(Z),
            _ => Err(ParseErr {
                input: input.to_string(),
                at: 0,
                expect: "W|X|Y|Z".to_string(),
                inner: None,
            }),
        }
    }
}

impl FromStr for Operand {
    type Err = ParseErr;

    fn from_str(input: &str) -> Result<Operand, Self::Err> {
        Register::from_str(input)
            .map(Operand::RegOp)
            .or_else(|_| input.parse::<usize>().map(Operand::Literal))
            .map_err(|e| ParseErr {
                input: input.to_string(),
                at: 0,
                expect: "operand".to_string(),
                inner: Some(Err(e)),
            })
    }
}

#[rustfmt::skip]
impl FromStr for Instr {
    type Err = ParseErr;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input.split_ascii_whitespace().collect_vec();        
        if parts.is_empty() {
            return Err(ParseErr { input: input.to_string(), at: 0, expect: "expected instr name".to_string(), inner: None, });
        }
        match parts[0] {
            "inp" => Register::from_str(parts[1]).map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                     .map(|reg| Instr::Inp { dst: reg }),
            "add" => Register::from_str(parts[1])
                .map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                .and_then(|dst| {
                    match Operand::from_str(parts[2]) {
                        Ok(operand) => Ok(Instr::Add { dst, operand }),
                        Err(e) => Err(ParseErr { input: input.to_string(), at: 2, expect: "expected operand".to_string(), inner: Some(Ok(Box::new(e))), }),
                    }
                }),
            "mul" => Register::from_str(parts[1])
                .map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                .and_then(|dst| {
                    match Operand::from_str(parts[2]) {
                        Ok(operand) => Ok(Instr::Mul { dst, operand }),
                        Err(e) => Err(ParseErr { input: input.to_string(), at: 2, expect: "expected operand".to_string(), inner: Some(Ok(Box::new(e))), }),
                    }
                }),
            "div" => Register::from_str(parts[1])
                .map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                .and_then(|dst| {
                    match Operand::from_str(parts[2]) {
                        Ok(operand) => Ok(Instr::Div { dst, operand }),
                        Err(e) => Err(ParseErr { input: input.to_string(), at: 2, expect: "expected operand".to_string(), inner: Some(Ok(Box::new(e))), }),
                    }
                }),
            "mod" => Register::from_str(parts[1])
                .map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                .and_then(|dst| {
                    match Operand::from_str(parts[2]) {
                        Ok(operand) => Ok(Instr::Mod { dst, operand }),
                        Err(e) => Err(ParseErr { input: input.to_string(), at: 2, expect: "expected operand".to_string(), inner: Some(Ok(Box::new(e))), }),
                    }
                }),
            "eql" => Register::from_str(parts[1])
                .map_err(|e| ParseErr { input: input.to_string(), at: 1, expect: "expected register".to_string(), inner: Some(Ok(Box::new(e))), })
                .and_then(|dst| {
                    match Operand::from_str(parts[2]) {
                        Ok(operand) => Ok(Instr::Eql { dst, operand }),
                        Err(e) => Err(ParseErr { input: input.to_string(), at: 2, expect: "expected operand".to_string(), inner: Some(Ok(Box::new(e))), }),
                    }
                }),                             
            _ => panic!("unknown instruction: {}", parts[0]),
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::ParseErr;
    use super::Instrs;
    use super::Instr::{self, Add, Div, Eql, Inp, Mod, Mul};
    use super::Operand::{self, Literal, RegOp};
    use super::Register::{W, X, Y, Z};
    use std::str::FromStr;

    fn parse_one(input: &str, trace: bool) -> Result<Instr, ParseErr> {
        if trace { eprintln!("parsing: {}", input); };
        let result = Instr::from_str(input);
        if trace { eprintln!("parsed: {:?}", result); }
        result
    }

    fn parse_many(input: &str, trace: bool) -> Result<Instrs, ParseErr> {
        let lines = input.lines();
        let mut instrs = vec![];
        for (i,line) in lines.enumerate() {
            match parse_one(line, trace) {
                Ok(instr) => instrs.push(instr),
                Err(e) => return Err(e)
            }
        }
        Ok(instrs)
    }

    #[test]
    fn parse_input() {
        let input = include_str!("../../inputs/day24.txt");
        parse_many(input, true);
    }
    
    #[test]
    fn parse_single() {
        assert_eq!(
            parse_one("inp x", true), Ok(Inp { dst: X })
        );
        assert_eq!(
            parse_one("add w w", true), Ok(Add { dst: W, operand: RegOp(W) })
        );
        assert_eq!(
            parse_one("add w 2", true), Ok(Add { dst: W, operand: Literal(2) })
        );
        assert_eq!(
            parse_one("mul x w", true), Ok(Mul { dst: X, operand: RegOp(W) })
        );
        assert_eq!(
            parse_one("div y z", true), Ok(Div { dst: Y, operand: RegOp(Z) })
        );
        assert_eq!(
            parse_one("mod z 9", true), Ok(Mod { dst: Z, operand: Literal(9) })
        );
        assert_eq!(
            parse_one("eql z w", true), Ok(Eql { dst: Z, operand: RegOp(W) })
        );
    }
}
