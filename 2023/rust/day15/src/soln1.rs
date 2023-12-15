use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<String>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    //todo ignore newlinelib
    let lines = input.lines().collect_vec();
    let seqs = lines.iter().map(|line| line.split(',').map(|x| x.to_string()).collect_vec());
    itertools::concat(seqs)
}

fn hash(str: String) -> u8 {
    let mut hash = 0u16;
    for c in str.chars() {
        let ascii = c as u16;
        hash += ascii;
        hash *= 17;
        hash = hash % 256;
    }
    hash as u8
    /*

       Determine the ASCII code for the current character of the string.
       Increase the current value by the ASCII code you just determined.
       Set the current value to itself multiplied by 17.
       Set the current value to the remainder of dividing itself by 256.
    */
}
// fn hash<H: Hasher>(&self, state: &mut H) {
//     let x = self.x.unsigned_abs() as u64;
//     let y = self.y.unsigned_abs() as u64;

//     /* szudziks function */
//     let hash_val = if x >= y { x * x + x + y } else { x + y * y };
//     state.write_u64(hash_val);
// }

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let mut total = 0;
    for str in input {
        let hashs = hash(str.to_string());
        total += (hashs as usize);
        println!("Hash {str:?} = {hashs}");
    }
    // println!("P1 Input = {input:?}");
    total
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    // let mut hashmap = HashMap::new();
    let mut hashmap: Vec<Vec<(usize, String)>> = vec![vec![]; 256];
    for str in input {
        let sclone = str.clone();
        let sp = sclone.split_inclusive(|char| char == '=' || char == '-').collect_vec();
        println!("splits: {sp:?}");
        let comb = sp[0].clone();
        let label = &comb[0..comb.len() - 1];
        let op = &comb[comb.len() - 1..];
        // let label = sp[0].clone();
        // let op = sp[1].clone();
        let idx: Option<usize> = if sp.len() > 1 { sp[1].clone().parse().ok() } else { None };

        let boxnum = hash(label.to_string()) as usize;
        if op == "=" {
            let cp = hashmap[boxnum].clone();
            let existing = cp.iter().enumerate().find(|(ix, (pwr, it))| it == label);
            if existing.is_some() {
                let (ei, (epwr, eit)) = existing.unwrap();
                hashmap[boxnum][ei] = (idx.unwrap(), eit.to_string());
                // hashmap[boxnum].remove(existing.unwrap().0);
            } else {
                hashmap[boxnum].push((idx.unwrap(), label.to_string()));
            }
        } else if op == "-" {
            let mut bucket = hashmap[boxnum].clone();
            bucket = bucket.into_iter().filter(|(ix, lbl)| *lbl != label).collect_vec();
            hashmap[boxnum] = bucket;
        }
    }
    /*

    One plus the box number of the lens in question.
    The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    The focal length of the lens.

     */
    println!("HASH MAP AT END = {hashmap:?}");
    let mut total = 0;
    for bucket in 0..256 {
        let mut bucket_num = bucket + 1;
        let entry = &hashmap[bucket];
        for eidx in 0..entry.len() {
            let (idx, item) = &entry[eidx];
            total += (bucket_num) * (eidx + 1) * idx
        }
    }
    total
    // todo!()
}
