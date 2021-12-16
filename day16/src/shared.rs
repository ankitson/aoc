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

fn extract_at(bytes: u16, mask: u16, len: usize, start: usize) -> u16 {
    //11111 - mask
    //43210 - bit indexes
    //this mask starts at idx 4 = len-1
    //to start at idx 7, we left shift (7-4) = 3 times
    let current_start = len - 1;
    if start < current_start {
        panic!("illegal start for mask")
    }
    let byte_num = start / 16;
    let byte_start = start % 16;
    ((mask << (start - current_start)) & bytes) >> (start - len - 1)
}

const PTYPE_LITERAL: u8 = 0b100;

pub fn parse_bv(input: &str) -> BitVec<Msb0, u8> {
    let bytes: Vec<u8> = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect_vec();
    BitVec::from_vec(bytes)
}

pub fn parse(input: &str) {
    let bv = parse_bv(input);
}

type BS = BitSlice<Msb0, u8>;

pub fn parse_packet(input: &BS) -> Option<(&BS, u64)> {
    let version = &input[0..3];
    let typeid = &input[3..6];
    if typeid == bits![1, 0, 0] {
        let (remaining, literal) = parse_literal(&input[6..]).unwrap();
        return Some((remaining, literal.reverse_bits()));
    }
    return None;
}

pub fn parse_literal(input: &BS) -> Option<(&BS, u64)> {
    println!("input: {:b}", input);
    let first = input[0];
    let half: u64 = input[1..5].load_le::<u64>();
    println!("half: {:04b}", half);
    if first {
        let (rem, lit) = parse_literal(&input[5..]).unwrap();
        let cval = (lit >> 4) | (half << (64 - 4));
        Some((&input[5..], cval))
    } else {
        Some((&input[5..], half))
    }
    // todo!()
}

// pub fn parse(input: &str) -> Packet {
//     let mut offset = 0;
//     let mut two_bytes = input.as_bytes().chunks(2).map(|chunk| -> u16 {
//         let b: [u8; 2] = match chunk.try_into() {
//             Ok(bb) => bb,
//             Err(_) => todo!(),
//         };
//         // let twob: [u8; 2] = *chunk.try_into().unwrap();
//         u16::from_be_bytes(b)
//         //u16::from_be_bytes(*chunk.try_into().unwrap())
//     });
//     let mut bytes = two_bytes.next().unwrap();
//     dbg!(bytes);
//     let version = extract_at(bytes, 0b111, 3, 7 + 8);
//     let ptype = extract_at(bytes, 0b111, 3, 4 + 8);
//     dbg!(version);
//     dbg!(ptype);
//     offset = 6;
//     if ptype == 4 {
//         let mut lit_bytes: Vec<u8> = Vec::new();
//         let mut current: u8 = 0;
//         let mut lower = false;
//         let mut mode: Option<bool> = None;
//         loop {
//             let first_bit = extract_at(bytes, 0b1, 1, 15 - offset);
//             offset += 1;
//             if offset >= 16 {
//                 bytes = two_bytes.next().unwrap()
//             }
//             let next4: u8 = extract_at(bytes, 0xF, 4, 15 - offset).try_into().unwrap();
//             println!("extraced literal 4bytes {:02x}", next4);
//             offset += 4;
//             if offset >= 16 {
//                 bytes = two_bytes.next().unwrap()
//             }
//             if !lower {
//                 current = current & (next4 << 4);
//             } else {
//                 current = current & next4;
//                 lit_bytes.push(current);
//                 current = 0;
//             }
//             lower = !lower;
//             if first_bit == 0 {
//                 if current != 0 {
//                     current = current >> 4;
//                     lit_bytes.push(current)
//                 }
//                 break;
//             }
//         }
//         println!("Parsed literal: {:?}", lit_bytes);
//     }

//     // let bytes = input.as_bytes().iter();
//     // let byte = bytes.next().unwrap();
//     todo!();
//     // let version = extract_at(bytes, 0b111, 3, 7)
// }

#[cfg(test)]
mod tests {
    use crate::shared::{parse_bv, parse_packet};
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
        println!("{:02x}", parsed);
        assert_eq!(parse_packet(&parse_bv(literal1)).unwrap().1, 2021u64)
    }
}

// fn parse_rec(input: &[char]) -> (&[char], T) {
//     let (rem, version) = parse_version(input);
//     let (rem, etype) = parse_etype(rem);
//     if etype.unwrap() == 4
// }

// fn parse_version(input: &[char]) -> (&[char], Option<PacketVersion>) {
//     if input.len() < 3 {
//         return (input, None);
//     }
//     let f3: &[char; 3] = &input[..3].try_into().unwrap();
//     (&input[3..], Some(PacketVersion(f3)))
// }

// fn parse_etype(input: &[char]) -> (&[char], Option<PacketType>) {
//     if input.len() < 3 {
//         return (input, None);
//     }
//     let f3: &[char; 3] = &input[..3].try_into().unwrap();
//     (&input[3..], Some(PacketType(f3)))
// }
