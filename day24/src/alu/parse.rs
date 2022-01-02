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
            .or_else(|_| input.parse::<isize>().map(Operand::Literal))
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

impl FromStr for Instrs {
    type Err = Vec<ParseErr>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines();
        let mut instrs = vec![];
        let mut errs = vec![];
        for (i, line) in lines.enumerate() {
            match Instr::from_str(line) {
                Ok(instr) => instrs.push(instr),
                Err(e) => errs.push(e),
            }
        }
        if !errs.is_empty() {
            Err(errs)
        } else {
            Ok(Instrs(instrs))
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

    fn parse_many(input: &str, trace: bool) -> Result<Instrs, Vec<ParseErr>> {
        let instrs = Instrs::from_str(input);
        if trace { println!("Parsed Instrs:\n{:#?}", instrs) }
        instrs
    }

    #[test]
    fn parse_input() {
        let input = include_str!("../../inputs/day24.txt");
        let instrs = parse_many(input, false).unwrap();
        assert_eq!(instrs.0.len(), 252);
        assert_eq!(instrs.0[0], Inp {dst: W });
        assert_eq!(instrs.0[95], Add {dst: X, operand: Literal(-2) });
        assert_eq!(*instrs.0.last().unwrap(), Add {dst: Z, operand: RegOp(Y)});
    }
    
    #[test]
    fn parse_single() {
        let trace = false;
        assert_eq!(
            parse_one("inp x", trace), Ok(Inp { dst: X })
        );
        assert_eq!(
            parse_one("add w w", trace), Ok(Add { dst: W, operand: RegOp(W) })
        );
        assert_eq!(
            parse_one("add w 2", trace), Ok(Add { dst: W, operand: Literal(2) })
        );
        assert_eq!(
            parse_one("mul x w", trace), Ok(Mul { dst: X, operand: RegOp(W) })
        );
        assert_eq!(
            parse_one("div y z", trace), Ok(Div { dst: Y, operand: RegOp(Z) })
        );
        assert_eq!(
            parse_one("mod z 9", trace), Ok(Mod { dst: Z, operand: Literal(9) })
        );
        assert_eq!(
            parse_one("eql z w", trace), Ok(Eql { dst: Z, operand: RegOp(W) })
        );
    }
}
