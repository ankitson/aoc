use std::hash::Hasher;

use itertools::Itertools;

pub type Input = Vec<&'static [u8]>;
pub type Output = usize;

pub fn parse(input: &'static str) -> std::slice::Split<'_, u8, impl FnMut(&u8) -> bool> {
    let bytes = input.as_bytes();
    let skip_last = &bytes[0..bytes.len() - 1];
    let seqs = skip_last.split(|x| *x == b'\n' || *x == b',');
    seqs
}

/// Determine the ASCII code for the current character of the string.
/// Increase the current value by the ASCII code you just determined.
/// Set the current value to itself multiplied by 17.
/// Set the current value to the remainder of dividing itself by 256.
fn hash(str: &[u8]) -> u8 {
    let mut hash = 0u8;
    for c in str {
        hash = hash.wrapping_add(*c);
        hash = hash.wrapping_mul(17);
        hash = hash & 255;
    }
    hash as u8
}

#[allow(unused)]
fn hash_dual(str: &[u8], hasher: &mut dyn Hasher) -> u8 {
    let mut hash = 0u8;
    for c in str {
        hasher.write_u8(*c);
        hash = hash.wrapping_add(*c);
        hash = hash.wrapping_mul(17);
        hash = hash & 255;
    }
    hash as u8
}

pub fn part1(raw_input: &'static str) -> Output {
    let input = parse(raw_input);

    let mut total = 0;
    for str in input {
        let hashs = hash(str);
        total += hashs as usize;
    }
    total
}

pub fn part2(raw_input: &'static str) -> Output {
    let input = parse(raw_input);
    const EMPTY_VEC: Vec<(u8, &[u8])> = Vec::new();
    let mut hashmap = [EMPTY_VEC; 256];
    for str in input {
        let (sep_idx, sep) = str.iter().find_position(|c| **c == b'-' || **c == b'=').unwrap();
        let label = &str[0..sep_idx];
        let boxnum = hash(label) as usize;
        let entry = &mut hashmap[boxnum];
        if *sep == b'=' {
            let pwr = (str[sep_idx + 1] - b'0') as u8;
            let posm = entry.iter().find_position(|t| t.1 == label).map(|x| x.0);
            if let Some(pos) = posm {
                entry[pos] = (pwr, label);
            } else {
                entry.push((pwr, &label));
            }
        } else {
            entry.retain(|e| e.1 != label);
        }
    }
    let mut total = 0;
    for bucket in 0..256 {
        let entries = &hashmap[bucket];
        for eidx in 0..entries.len() {
            let (pwr, _) = &entries[eidx];
            total += (bucket + 1) * (eidx as usize + 1) * (*pwr as usize)
        }
    }
    total
}
