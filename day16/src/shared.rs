use bitvec::prelude::*;
use itertools::Itertools;

pub fn parse_bv(input: &str) -> BitVec<Msb0, u8> {
    let bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect_vec();
    BitVec::from_vec(bytes)
}

type BS = BitSlice<Msb0, u8>;

fn bitslice_to_lit(bits: &BS) -> u64 {
    if (bits.len() > 64) {
        panic!("bits too long for a u64")
    }

    let mut result: u64 = 0;
    for bit in bits.iter() {
        let b: u64 = {
            if *bit {
                1
            } else {
                0
            }
        };
        result |= b;
        result <<= 1;
    }
    result >>= 1;
    result
}

pub fn parse_packet<'a>(input: &'a BS, vcounter: &mut u64) -> Option<(&'a BS, u64)> {
    if input.len() < 3 {
        return None;
    }
    let versionb = &input[0..3];
    let version = bitslice_to_lit(versionb);
    *vcounter += version;
    let typeid = bitslice_to_lit(&input[3..6]);
    if typeid == 4 {
        let (remaining, literal) = parse_literal(&input[6..]).unwrap();
        let lvalue = bitslice_to_lit(literal[..].into());
        return Some((remaining, lvalue));
    } else {
        let ltypeid = input[6];
        let mut subvals: Vec<u64> = Vec::new();
        let mut rem: &BS = &input[7..];
        if ltypeid == true {
            //parse fixed number of PACKETS
            let numsub = bitslice_to_lit(&input[7..7 + 11]);
            let p = parse_packet(&input[7 + 11..], vcounter);
            let (remp, mut lit) = p.unwrap();
            rem = remp;
            subvals.push(lit);
            for _ in 0..numsub - 1 {
                let p = parse_packet(rem, vcounter).unwrap();
                rem = p.0;
                lit = p.1;
                subvals.push(lit);
            }
        } else {
            //parse fixed number of BITS
            let totallen: usize = bitslice_to_lit(&input[7..7 + 15]).try_into().unwrap();
            rem = &input[7 + 15..];
            let mut reml = rem.len();
            let mut lit: u64 = 0;
            let mut consoomlen = 0usize;
            while consoomlen < totallen {
                let p = parse_packet(rem, vcounter);
                let q = p.unwrap();
                rem = q.0;
                lit = q.1;
                subvals.push(lit);
                consoomlen += reml - rem.len();
                reml = rem.len();
            }
        }
        match typeid {
            0 => return Some((rem, subvals.iter().sum())),
            1 => return Some((rem, subvals.iter().product())),
            2 => return Some((rem, *subvals.iter().min().unwrap())),
            3 => return Some((rem, *subvals.iter().max().unwrap())),
            5 => {
                let v = if subvals[0] > subvals[1] { 1u64 } else { 0u64 };
                return Some((rem, v));
            }
            6 => {
                let v = if subvals[0] < subvals[1] { 1u64 } else { 0u64 };
                return Some((rem, v));
            }
            7 => {
                let v = if subvals[0] == subvals[1] { 1u64 } else { 0u64 };
                return Some((rem, v));
            }
            _ => panic!("unknown op"),
        }
    }
}

pub fn parse_literal(input: &BS) -> Option<(&BS, BitVec<Msb0, u8>)> {
    let first = input[0];
    let mut lit: BitVec<Msb0, u8> = bitvec![Msb0, u8;];
    lit.extend(&input[1..5]);
    if first {
        let (rem, tailn) = parse_literal(&input[5..]).unwrap();
        lit.extend(tailn);
        Some((rem, lit))
    } else {
        Some((&input[5..], lit))
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{parse_bv, parse_packet};

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
    fn test_parse_op_vsum() {
        let ops = vec![
            "38006F45291200",
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        static mut vcounter: u64 = 0u64;

        let vsums = vec![9, 16, 12, 23, 31];
        for i in 0..5 {
            println!("Test parsing: {}", ops[i]);
            let bv = parse_bv(ops[i]);
            unsafe {
                vcounter = 0;
                let parsed = parse_packet(&bv, &mut vcounter);
                println!("Parsed: {}", parsed.unwrap().1);
                assert_eq!(vcounter, vsums[i]);
            }
        }
    }

    #[test]
    fn test_parse_op() {
        let msgs = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        static mut vcounter: u64 = 0u64;
        for i in 0..5 {
            println!("Test parsing: {}", msgs[i].0);
            let bv = parse_bv(msgs[i].0);
            unsafe {
                let parsed = parse_packet(&bv, &mut vcounter);
                let evaled = parsed.unwrap().1;
                println!("Evaluated = {}", evaled);
                assert_eq!(evaled, msgs[i].1);
            }
        }
    }
}
