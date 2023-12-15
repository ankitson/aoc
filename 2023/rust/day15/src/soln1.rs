use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<&'static [u8]>;
pub type Output = usize;

pub fn parse(input: &'static str) -> Input {
    let bytes = input.as_bytes();
    let seqs = bytes.split(|x| *x == b'\n' || *x == b',').collect_vec();
    seqs
}

/// Determine the ASCII code for the current character of the string.
/// Increase the current value by the ASCII code you just determined.
/// Set the current value to itself multiplied by 17.
/// Set the current value to the remainder of dividing itself by 256.
fn hash(str: &[u8]) -> u8 {
    let mut hash = 0u16;
    for c in str {
        hash += *c as u16;
        hash *= 17;
        hash = hash % 256;
    }
    hash as u8
}

pub fn part1(raw_input: &'static str) -> Output {
    let input = parse(raw_input);
    let mut total = 0;
    for str in input {
        let hashs = hash(str);
        total += (hashs as usize);
    }
    total
}

pub fn part2(raw_input: &str) -> Output {
    todo!();
    // let input = parse(raw_input);
    // let mut hashmap: Vec<Vec<(usize, String)>> = vec![vec![]; 256];
    // for str in input {
    //     let sclone = str.clone();
    //     let sp = sclone.split_inclusive(|char| char == '=' || char == '-').collect_vec();
    //     let comb = sp[0];
    //     let label = &comb[0..comb.len() - 1];
    //     let op = &comb[comb.len() - 1..];

    //     let idx: Option<usize> = if sp.len() > 1 { sp[1].parse().ok() } else { None };

    //     let boxnum = hash(label.to_string()) as usize;
    //     if op == "=" {
    //         let cp = hashmap[boxnum].clone();
    //         let existing = cp.iter().enumerate().find(|(_, (_, it))| it == label);
    //         if existing.is_some() {
    //             let (ei, (_, eit)) = existing.unwrap();
    //             hashmap[boxnum][ei] = (idx.unwrap(), eit.to_string());
    //         } else {
    //             hashmap[boxnum].push((idx.unwrap(), label.to_string()));
    //         }
    //     } else if op == "-" {
    //         let mut bucket = hashmap[boxnum].clone();
    //         bucket = bucket.into_iter().filter(|(_, lbl)| *lbl != label).collect_vec();
    //         hashmap[boxnum] = bucket;
    //     }
    // }
    // let mut total = 0;
    // for bucket in 0..256 {
    //     let entry = &hashmap[bucket];
    //     for eidx in 0..entry.len() {
    //         let (idx, _) = &entry[eidx];
    //         total += (bucket + 1) * (eidx + 1) * idx
    //     }
    // }
    // total
}
