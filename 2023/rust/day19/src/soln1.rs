use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use itertools::Itertools;
use regex::Regex;

pub type Input = (HashMap<String, Vec<String>>, Vec<Vec<(char, usize)>>);
pub type Output = usize;

pub fn parse(input: &str) -> (HashMap<String, Vec<String>>, Vec<Vec<(char, usize)>>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut instrs = HashMap::new();
    let mut parts = vec![];

    for line in p1.lines() {
        if line.len() < 2 {
            continue;
        }
        let s1 = line.split('{').collect_vec();
        let wname = s1[0];
        let wdescr = &s1[1][0..s1[1].len() - 1];
        let rules = wdescr.split(",").map(|x| x.to_string()).collect_vec();
        instrs.insert(wname.to_string(), rules);
    }
    for line in p2.lines() {
        if line.len() < 2 {
            continue;
        }
        let l2 = &line[1..line.len() - 1];
        let attrs = l2.split(",");
        let attrparse = attrs
            .map(|z| {
                let zchs = z.chars().collect_vec();
                (zchs[0], z[2..z.len()].parse::<usize>().unwrap())
            })
            .collect_vec();
        parts.push(attrparse);
    }
    (instrs, parts)
}

fn apply(instr: &Vec<String>, part: &Vec<(char, usize)>) -> String {
    for instr_part in instr {
        let parts = instr_part.split(":").collect_vec();
        if parts.len() == 1 {
            return parts[0].to_string();
            // todo!() //A , R or jump
        } else {
            let cond = parts[0];
            let label = parts[1];
            if cond.contains("<") {
                let pts = cond.split("<").collect_vec();
                let cmpwith = pts[0];
                let cmpvalu = pts[1].parse::<usize>().unwrap();
                let part_valu = part.iter().filter(|(a, b)| a.to_string() == cmpwith).map(|(a, b)| b).collect_vec()[0];
                if part_valu < &cmpvalu {
                    return label.to_string();
                }
            } else if cond.contains(">") {
                let pts = cond.split(">").collect_vec();
                let cmpwith = pts[0];
                let cmpvalu = pts[1].parse::<usize>().unwrap();
                let part_valu = part.iter().filter(|(a, b)| a.to_string() == cmpwith).map(|(a, b)| b).collect_vec()[0];
                if part_valu > &cmpvalu {
                    return label.to_string();
                }
            } else {
                panic!()
            }
        }
    }
    "illegal".to_string()
}

pub fn part1(raw_input: &str) -> Output {
    let (instrs, parts) = parse(raw_input);

    let mut accepted_total: usize = 0;
    for part in parts {
        let mut instr: &Vec<String> = instrs.get("in").unwrap();
        let mut next_label = "in".to_string();
        loop {
            next_label = apply(instr, &part);
            if next_label == "A" {
                let part_value = part.iter().map(|x| x.1);
                let part_sum = part_value.sum::<usize>();
                accepted_total += part_sum;
                break;
            } else if next_label == "R" {
                break;
            } else {
                instr = instrs.get(&next_label).unwrap();
            }
        }
    }
    accepted_total
}

fn order(r1: Range<usize>, r2: Range<usize>) -> (Range<usize>, Range<usize>) {
    if r1.start <= r2.start {
        return (r1, r2);
    } else {
        return (r2, r1);
    }
}
fn range_union(r1: Range<usize>, r2: Range<usize>) -> Vec<Range<usize>> {
    let (rs, rb) = order(r1, r2);
    if rs.end <= rb.start {
        //disjoint ranges
        return vec![rs, rb];
    } else {
        //intersect or rb is wholly contained in rs
        return vec![Range { start: rs.start, end: rb.end.max(rs.end) }];
    }
}

pub fn part2(raw_input: &str) -> Output {
    let (mut instrs, parts) = parse(raw_input);

    let mut left = 0;
    let mut right = 0;
    let mut instr = instrs.get("in").unwrap();

    let mut process = VecDeque::from([("in", vec![1..4001, 1..4001, 1..4001, 1..4001])]);

    let charmap = HashMap::from([('x', 0), ('m', 1), ('a', 2), ('s', 3)]);
    let mut count = 0usize;
    instrs.insert("A".to_string(), vec!["A".to_string()]);
    instrs.insert("R".to_string(), vec!["R".to_string()]);
    while process.len() > 0 {
        let (instr_label, mut ranges_per_symbol) = process.pop_front().unwrap();
        println!("process instr: {instr_label} ranges: {ranges_per_symbol:?}");
        let instr_parts = instrs.get(instr_label).unwrap();
        for part in instr_parts {
            println!("\tpart: {part:?} ranges {ranges_per_symbol:?}");
            let branches = part.split(":").collect_vec();
            match &branches[..] {
                [dest] if *dest == "A" => {
                    count += ranges_per_symbol.iter().map(|range| range.end - range.start).product::<usize>();
                }
                [dest] if *dest == "R" => (),
                [dest] => {
                    process.push_back((dest, ranges_per_symbol.clone()));
                }
                [cond, dest] => {
                    let cb = cond.as_bytes();
                    let operand = cb[0] as char;
                    let comp = cb[1] as char;
                    let valu: usize = std::str::from_utf8(&cb[2..]).unwrap().parse().unwrap();
                    let range = &ranges_per_symbol[*charmap.get(&operand).unwrap()];
                    if comp == '<' {
                        let n = Ord::clamp(valu, range.start, range.end);
                        let lt = range.start..n;
                        let gteq = n..range.end;
                        let mut new_ranges = ranges_per_symbol.clone();
                        new_ranges[*charmap.get(&operand).unwrap()] = lt;
                        process.push_back((dest, new_ranges));
                        ranges_per_symbol[*charmap.get(&operand).unwrap()] = gteq;
                    } else if comp == '>' {
                        let n = Ord::clamp(valu + 1, range.start, range.end);
                        let lteq = range.start..n;
                        let gt = n..range.end;
                        let mut new_ranges = ranges_per_symbol.clone();
                        new_ranges[*charmap.get(&operand).unwrap()] = gt;
                        process.push_back((dest, new_ranges));
                        ranges_per_symbol[*charmap.get(&operand).unwrap()] = lteq;
                    } else {
                        panic!();
                    }
                }
                other => (),
            }
        }
    }
    count
}

// for instr_part in instr {
//     let parts = instr_part.split(":").collect_vec();
//     if parts.len() == 1 {
//         right =
//         // todo!() //A , R or jump
//     } else {
//         let cond = parts[0];
//         let label = parts[1];
//         if cond.contains("<") {
//             let pts = cond.split("<").collect_vec();
//             let cmpwith = pts[0];
//             let cmpvalu = pts[1].parse::<usize>().unwrap();
//             let part_valu = part.iter().filter(|(a, b)| a.to_string() == cmpwith).map(|(a, b)| b).collect_vec()[0];
//             if part_valu < &cmpvalu {
//                 return label.to_string();
//             }
//         } else if cond.contains(">") {
//             let pts = cond.split(">").collect_vec();
//             let cmpwith = pts[0];
//             let cmpvalu = pts[1].parse::<usize>().unwrap();
//             let part_valu = part.iter().filter(|(a, b)| a.to_string() == cmpwith).map(|(a, b)| b).collect_vec()[0];
//             if part_valu > &cmpvalu {
//                 return label.to_string();
//             }
//         } else {
//             panic!()
//         }
//     }

// px -> A
//    -> rfg

// rfg -> gd (all p such that s < 537 i.e 536 * 4000 * 4000 * 4000)
//     -> R (all p such that s>= 537 and x>2440 i.e (4000-536) *
