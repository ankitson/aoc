use bitvec::prelude::*;
use itertools::Itertools;

/**
 * Packet description:
 *
 * HHHHHH....
 * VVVTTT....
 *
 * H = header = VVVTTT
 *    VVV: packet version
 *    TTT: packet type
 *
 * TTT = 100 = 4 -> literal value packet
 *
 *
 */
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

fn bitvec_to_lit(bitvec: BitVec) -> u64 {
    let mut result: u64 = 0;
    for bit in bitvec {
        let b: u64 = {
            if bit {
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

pub fn parse_packet(input: &BS) -> Option<(&BS, u64)> {
    let version = &input[0..3];
    let typeid = &input[3..6];
    if typeid == bits![1, 0, 0] {
        let (remaining, literal) = parse_literal(&input[6..]).unwrap();
        return Some((remaining, bitvec_to_lit(literal)));
    }
    return None;
}

pub fn parse_literal(input: &BS) -> Option<(&BS, BitVec)> {
    println!("input: {:b}", input);
    let first = input[0];
    let mut lit = bitvec![];
    lit.extend(&input[1..5]);
    if first {
        let (rem, tailn) = parse_literal(&input[5..]).unwrap();
        lit.extend(tailn);
        Some((&input[5..], lit))
    } else {
        Some((&input[5..], lit))
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{bitvec_to_lit, parse_bv, parse_packet};
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
        let ops = vec![
            "38006F45291200",
            "EE00D40C823060",
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        let (rem, parsed) = parse_packet(&parse_bv(literal1)).unwrap();
        println!("parsed literal: {:b}", parsed);
        assert_eq!(parsed, 2021u64)
    }
}
