use itertools::Itertools;
use std::{
    collections::{BinaryHeap, VecDeque},
    ops::Index,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PEH {
    Leaf(usize),
    Node(Box<PEH>, Box<PEH>),
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PEHQ {
    peh: PEH,
    priority: usize,
}

impl Ord for PEHQ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PEHQ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Desired structure..
///
/// [[6,[5,[4,[3,2]]]], 1]
/// becomes (num,depth) pairs
/// [(6,1),(5,2),(4,3),(3,4),(2,4),(1,0)]
///
/// now explode: find depth 4 pair and replace
///

pub fn parse(input: &str) -> PEH {
    let chars = input.chars().collect_vec();
    let mut vec: Vec<(usize, usize)> = Vec::new();
    let node = parse_rec(&chars, 0, &mut vec);
    println!("parsed: {:?}", node);
    todo!()
}

pub fn parse_rec(chars: &[char], depth: usize, vec: &mut Vec<(usize, usize)>) -> usize {
    println!("{}parse: {:?}", ".".repeat(depth), chars);
    if chars.is_empty() {
        return 0;
    }

    if chars[0].is_numeric() {
        vec.push((chars[0].to_digit(10).unwrap().try_into().unwrap(), depth));
        return 1;
        // parse_rec(&chars[1..], depth, vec);
    }

    if chars[0] == '[' {
        let np = parse_rec(&chars[1..], depth + 1, vec);
        parse_rec(&chars[np + 1..], depth + 1, vec)
    } else if chars[0] == ']' {
        parse_rec(&chars[1..], depth - 1, vec)
    } else {
        parse_rec(&chars[1..], depth, vec)
    }

    // let mut depth = 0;
    // let mut q: BinaryHeap<PEHQ> = BinaryHeap::new();
    // let mut cur: Vec<usize> = Vec::new();
    // let mut i = 0;
    // while i < chars. {
    //     let ch = chars.next().unwrap();
    //     if ch == '[' {
    //         depth += 1;
    //         continue;
    //     } else if ch == ']' {
    //         if cur.len() == 2 {
    //             let node = PEH { }
    //         }
    //         depth -= 1;
    //         // let
    //     } else if ch == ',' {
    //         let next = parse(input)
    //         continue;
    //     } else {
    //         let digit = ch.to_digit(10).unwrap();

    //     }
    // }
    // todo!(
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    //notes
    // i spent too long thinking of how to parse before thinking about what structure i want to parse to.
    // a tree structure is natural, but not super convenient for this problem
    use super::{parse, parse_rec};
    #[test]
    fn test_parse() {
        let mut vec = Vec::new();
        let strs = ["6", "[1,2]", "[1,[2,[3,[4,4]]],1]"];
        for str in strs {
            let mut chars = str.chars().collect_vec();
            parse_rec(&chars, 0, &mut vec);
            println!("str: {} parsed: {:?}", str, &vec);
            vec.clear();
        }
        // sample: &str = "[[6,[5,[4,[3,2]]]],1]";

        // parse_rec(&sample.chars().collect_vec(), 0, &mut vec);
        // assert_eq!(vec, vec![(6, 2), (5, 3), (4, 4), (3, 5), (2, 5), (1, 1)])

        // assert_eq!(parse(sample), ((20, 30), (-10, -5)))
    }
}
