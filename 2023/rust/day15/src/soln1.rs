use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use regex::Regex;

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
        hash = hash.wrapping_add(*c); // as u16;
        hash = hash.wrapping_mul(17);
        // hash = hash % 256;
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
    // let mut hashmap: Vec<Vec<(u8, u8, u8, &[u8])>> = vec![vec![]; 256]; //each bucket has list of (power,label) lenses
    const EMPTY_VEC: Vec<(u8, u8, u8, &[u8])> = Vec::new();
    let mut hashmap = [EMPTY_VEC; 256];
    let mut insertion_time = 0u8;
    // let mut total = 0;
    for str in input {
        // let strchars = String::from_utf8(str.to_vec());
        let (sep_idx, sep) = str.iter().find_position(|c| **c == b'-' || **c == b'=').unwrap();
        let label = &str[0..sep_idx];
        let boxnum = hash(label) as usize;
        if *sep == b'=' {
            let pwr = (str[sep_idx + 1] - b'0') as u8;
            let entry = &mut hashmap[boxnum];
            let idx_in = entry.len() as u8;
            entry.push((insertion_time, idx_in, pwr, &label)); //later entries are authoritative
                                                               // total += (boxnum + 1) * (idx_in as usize + 1) * (pwr as usize);
        } else {
            let entry = &mut hashmap[boxnum];
            entry.push((insertion_time, entry.len() as u8, 0, &label));
        }
        insertion_time += 1;
    }

    println!("FINAL HASHMAP: {hashmap:?}");
    let mut total = 0;
    // let mut seen = BTreeSet::new();
    total
    // for bucket in 0..256 {
    //     // let (bef, aft) = hashmap.split_at_mut(bucket);
    //     let entries = &mut hashmap[bucket];
    //     // let entries = &mut hashmap[bucket];
    //     entries.sort();
    //     for eidx in (0..entries.len()).rev() {
    //         let (insert_time, insert_idx, pwr, lbl) = &entries[eidx];
    //         if !seen.contains(lbl) {
    //             // last_idx.insert(lbl, eidx);
    //             total += (bucket + 1) * (*insert_idx as usize + 1) * (*pwr as usize);
    //             seen.insert(lbl);
    //         }
    //     }
    //     seen.clear()
    // }
    // total
}
