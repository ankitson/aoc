use std::{arch::aarch64::*, fmt::Debug};

pub fn part_1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;

    unsafe {
        let mut last_m = vdupq_n_u8(0);
        let mut last_u = vdupq_n_u8(0);
        let mut last_l = vdupq_n_u8(0);
        for i in (0..input.len()).step_by(16) {
            let chunk = *(input.as_ptr().add(i) as *const uint8x16_t);
            let m = vceqq_u8(chunk, vdupq_n_u8(b'm'));
            let u = vceqq_u8(chunk, vdupq_n_u8(b'u'));
            let l = vceqq_u8(chunk, vdupq_n_u8(b'l'));
            let paren = vceqq_u8(chunk, vdupq_n_u8(b'('));
            let mut starts = vandq_u8(paren, vextq_u8::<15>(last_l, l));
            starts = vandq_u8(starts, vextq_u8::<14>(last_u, u));
            starts = vandq_u8(starts, vextq_u8::<13>(last_m, m));
            last_m = m;
            last_u = u;
            last_l = l;

            let mut starts_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
                vreinterpretq_u16_u8(starts),
            )));
            loop {
                let offset = starts_bits.trailing_zeros() / 4;
                if offset == 16 {
                    break;
                }
                starts_bits &= !(0xF << (offset * 4));
                let mut scalar_i = i + offset as usize + 1;
                let Some(a) = parse_num(input, &mut scalar_i, b',') else {
                    continue;
                };
                let Some(b) = parse_num(input, &mut scalar_i, b')') else {
                    continue;
                };
                sum += a * b;
            }
        }
    }
    sum
}

fn p1_verify(input: &str) -> impl Iterator<Item = (u32, u32)> {
    let input = input.as_bytes();
    let mut i = 0;
    let mut muls = vec![];
    while i < input.len() {
        if input.get(i..i + 4) == Some(b"mul(") {
            i += 4;
            let Some(a) = parse_num(input, &mut i, b',') else {
                continue;
            };
            let Some(b) = parse_num(input, &mut i, b')') else {
                continue;
            };
            muls.push((a, b));
        } else {
            i += 1;
        }
    }
    muls.into_iter()
}

pub fn part_2(input: &str) -> impl Debug {
    let input = input.as_bytes();
    let mut sum = 0;
    unsafe {
        let mut last_m = vdupq_n_u8(0);
        let mut last_u = vdupq_n_u8(0);
        let mut last_l = vdupq_n_u8(0);
        let mut last_lparen = vdupq_n_u8(0);
        let mut last_d = vdupq_n_u8(0);
        let mut last_o = vdupq_n_u8(0);
        let mut last_n = vdupq_n_u8(0);
        let mut last_apos = vdupq_n_u8(0);
        let mut last_t = vdupq_n_u8(0);

        let mut enabled = true;

        for i in (0..input.len()).step_by(16) {
            let chunk = *(input.as_ptr().add(i) as *const uint8x16_t);

            let d = vceqq_u8(chunk, vdupq_n_u8(b'd'));
            let o = vceqq_u8(chunk, vdupq_n_u8(b'o'));
            let lparen = vceqq_u8(chunk, vdupq_n_u8(b'('));
            let rparen = vceqq_u8(chunk, vdupq_n_u8(b')'));
            let n = vceqq_u8(chunk, vdupq_n_u8(b'n'));
            let apos = vceqq_u8(chunk, vdupq_n_u8(b'\''));
            let t = vceqq_u8(chunk, vdupq_n_u8(b't'));
            let partial_dos = vandq_u8(rparen, vextq_u8::<15>(last_lparen, lparen));

            let mut enabled_mul_mask = if enabled { u64::MAX } else { 0 };
            if !enabled {
                let mut dos = vandq_u8(partial_dos, vextq_u8::<14>(last_o, o));
                dos = vandq_u8(dos, vextq_u8::<13>(last_d, d));

                let dos_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
                    vreinterpretq_u16_u8(dos),
                )));

                assert!(dos_bits.count_ones() / 4 <= 1);

                let do_begin_offset = dos_bits.trailing_zeros() / 4;
                if do_begin_offset != 16 {
                    enabled_mul_mask |= u64::MAX << (do_begin_offset * 4);
                    enabled = true;
                }
            }
            if enabled {
                let mut donts = vandq_u8(partial_dos, vextq_u8::<14>(last_t, t));
                donts = vandq_u8(donts, vextq_u8::<13>(last_apos, apos));
                donts = vandq_u8(donts, vextq_u8::<12>(last_n, n));
                donts = vandq_u8(donts, vextq_u8::<11>(last_o, o));
                donts = vandq_u8(donts, vextq_u8::<10>(last_d, d));

                let donts_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
                    vreinterpretq_u16_u8(donts),
                )));
                assert!(donts_bits.count_ones() / 4 <= 1);
                let dont_begin_offset = donts_bits.trailing_zeros() / 4;
                if dont_begin_offset != 16 && enabled {
                    enabled_mul_mask &= !(u64::MAX << (dont_begin_offset * 4));
                    enabled = false;
                }
            }
            last_d = d;
            last_o = o;
            last_n = n;
            last_apos = apos;
            last_t = t;

            if enabled_mul_mask != 0 {
                let m = vceqq_u8(chunk, vdupq_n_u8(b'm'));
                let u = vceqq_u8(chunk, vdupq_n_u8(b'u'));
                let l = vceqq_u8(chunk, vdupq_n_u8(b'l'));
                let mut mul_starts = vandq_u8(lparen, vextq_u8::<15>(last_l, l));
                mul_starts = vandq_u8(mul_starts, vextq_u8::<14>(last_u, u));
                mul_starts = vandq_u8(mul_starts, vextq_u8::<13>(last_m, m));
                last_m = m;
                last_u = u;
                last_l = l;

                let mut mul_starts_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(
                    vshrn_n_u16::<4>(vreinterpretq_u16_u8(mul_starts)),
                ));
                mul_starts_bits &= enabled_mul_mask;

                loop {
                    let offset = mul_starts_bits.trailing_zeros() / 4;
                    if offset == 16 {
                        break;
                    }
                    mul_starts_bits &= !(0xF << (offset * 4));
                    let mut scalar_i = i + offset as usize + 1;
                    let Some(a) = parse_num(input, &mut scalar_i, b',') else {
                        continue;
                    };
                    let Some(b) = parse_num(input, &mut scalar_i, b')') else {
                        continue;
                    };
                    sum += a * b;
                }
            }
            last_lparen = lparen;
        }
    }
    sum
}

fn parse_num(input: &[u8], i: &mut usize, terminator: u8) -> Option<u32> {
    let mut num = 0;
    for j in 0..4 {
        // Return None when out of bounds because there always needs to be a character
        // after the number
        let character = *input.get(*i)?;
        if character == terminator {
            let res = (j != 0).then_some(num);
            *i += 1;
            return res;
        } else if j == 3 {
            return None;
        }
        let digit = u32::from(character.wrapping_sub(b'0'));
        if digit > 9 {
            return None;
        }
        num = digit + num * 10;
        *i += 1;
    }
    None
}
