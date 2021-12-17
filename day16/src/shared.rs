use bitvec::prelude::*;
use itertools::Itertools;
pub struct Packet<'a> {
    version: PacketVersion,
    ptype: PacketType<'a>,
    size: PacketSize<'a>,
    subpackets: Vec<Box<Packet<'a>>>,
}

pub struct PacketVersion(u8); //&'a [char; 3]);
pub struct PacketType<'a>(&'a [char; 3]);
pub enum PacketSize<'a> {
    Length(&'a [char; 15]),
    Count(&'a [char; 11]),
}

pub struct OpaquePacket {}

pub fn parse_bv(input: &str) -> BitVec<Msb0, u8> {
    let bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect_vec();
    BitVec::from_vec(bytes)
}

type BS = BitSlice<Msb0, u8>;

fn bitslice_to_lit(bits: &BS) -> u64 {
    let mut result: u64 = 0;
    for bit in bits.iter() {
        let b: u64 = {
            if *bit {
                1
            } else {
                0
            }
        };
        result = result | b;
        result <<= 1;
    }
    result >>= 1;
    result
}

pub fn parse_packet<'a>(input: &'a BS, vcounter: &'static mut u64) -> Option<(&'a BS, u64)> {
    println!("parsing {:?}", input);
    if input.len() < 3 {
        return None;
    }
    let versionb = &input[0..3];
    let version = bitslice_to_lit(versionb);
    println!("version: {}", version);
    unsafe {
        *vcounter += version;
    }
    let typeid = &input[3..6];
    if typeid == bits![1, 0, 0] {
        let (remaining, literal) = parse_literal(&input[6..]).unwrap();
        return Some((remaining, bitslice_to_lit(literal[..].into())));
    } else {
        let ltypeid = input[6];
        println!("ltype = {}", ltypeid);
        if ltypeid == true {
            //numsub = nmber of sub packets to parse
            let numsub = bitslice_to_lit(&input[7..7 + 11]);
            println!("parsing {} subpackets", numsub);
            //PROBLEM: Wont compile. borrow checker unhappy :/
            let p = parse_packet(&input[7 + 11..], vcounter).unwrap();
            let (mut rem, mut lit) = p;
            for _ in 0..numsub - 1 {
                let p = parse_packet(rem, vcounter).unwrap();
                rem = p.0;
                lit = p.1;
            }
        } else {
            let totallen: usize = bitslice_to_lit(&input[7..7 + 15]).try_into().unwrap();
            println!("parsing next {} bits", totallen);
            let mut rem = &input[7 + 15..];
            let mut reml = rem.len();
            let mut consoomlen = 0usize;
            while consoomlen < totallen {
                let p = parse_packet(rem, vcounter).unwrap();
                rem = p.0;
                // lit = p.1;
                consoomlen += (reml - rem.len());
                println!("consoomed {} bits", consoomlen);
                reml = rem.len();
            }
            // parse_packet(&input[7 + 15..]);
        }
    }
    return None;
}

pub fn parse_literal(input: &BS) -> Option<(&BS, BitVec<Msb0, u8>)> {
    println!("parse literal. input: {:?}", input);
    let first = input[0];
    let mut lit: BitVec<Msb0, u8> = bitvec![Msb0, u8;];
    lit.extend(&input[1..5]);
    if first {
        let (rem, tailn) = parse_literal(&input[5..]).unwrap();
        lit.extend(tailn);
        println!("parsed literal chunk: {:?}", lit);
        Some((rem, lit))
    } else {
        println!("parsed literal chunk: {:?}", lit);
        Some((&input[5..], lit))
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{bitslice_to_lit, parse_bv, parse_packet};
    use bitvec::prelude::*;
    use itertools::Itertools;

    #[test]
    fn test_parsebv() {
        let literal1 = "D2FE28";
        let ops = vec![
            "38006F45291200",
            "EE00D40C823060",
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        //1101 0010 1111 1110 0010 1000
        //VVVT TTMH HHHM HHHH MHHH HZZZ
        assert_eq!(parse_bv(literal1).as_raw_slice(), &[0xD2, 0xFE, 0x28]);
        assert_eq!(
            parse_bv(ops[0]).as_raw_slice(),
            &[0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00]
        );
    }

    #[test]
    fn test_parse_literal() {
        let literal1 = "D2FE28";

        let bv = parse_bv(literal1);
        static mut vcounter: u64 = 0u64;
        unsafe {
            let (rem, parsed) = parse_packet(&bv, &mut vcounter).unwrap();
            println!("parsed literal: {:b}", parsed);
            println!("rem: {:?}", rem.len());
            assert_eq!(vcounter, 6u64);
            assert_eq!(parsed, 2021u64);
        }
    }

    #[test]
    fn test_parse_op() {
        let ops = vec![
            "38006F45291200",
            "EE00D40C823060",
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        let bv = parse_bv(ops[0]);
        static mut vcounter: u64 = 0u64;
        unsafe {
            let (rem, parsed) = parse_packet(&bv, &mut vcounter).unwrap();
        }
    }
}
