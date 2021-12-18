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
    // let chars = input.chars().collect_vec();
    // let mut vec: Vec<(usize, usize)> = Vec::new();
    // let node = parse_rec(&chars, 0, &mut vec);
    // println!("parsed: {:?}", node);
    todo!()
}

fn parse_comma(chars: &[char], depth: usize, label: String) -> usize {
    println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);

    if chars.is_empty() || chars[0] != ',' {
        panic!()
    }
    1
}

fn parse_open(chars: &[char], depth: usize, label: String) -> usize {
    println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || chars[0] != '[' {
        panic!()
    }
    1
}

fn parse_close(chars: &[char], depth: usize, label: String) -> usize {
    println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || chars[0] != ']' {
        panic!()
    }
    1
}

fn parse_num(chars: &[char], depth: usize, label: String) -> (usize, usize) {
    println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || !chars[0].is_numeric() {
        panic!()
    }
    (chars[0].to_digit(10).unwrap().try_into().unwrap(), 1)
}

/// EXPR = [EXPR,EXPR]
/// EXPR = LIT
/// "[1,[2,[3,[4,4]]],1]"
pub fn parse_rec(chars: &[char], depth: usize, vec: &mut Vec<(usize, usize)>, label: String) -> usize {
    println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() {
        return 0;
    }

    if chars[0].is_numeric() {
        let (n, np) = parse_num(&chars[0..1], depth, "lit".to_string());
        vec.push((n, depth));
        1
    } else {
        let mut np = 0;
        np += parse_open(chars, depth, "open".to_string());
        let npleft = parse_rec(&chars[np..], depth + 1, vec, "left".to_string());
        np += npleft;
        np += parse_comma(&chars[np..], depth + 1, "comma".to_string());
        let npright = parse_rec(&chars[np..], depth + 1, vec, "right".to_string());
        np += npright;
        np += parse_close(&chars[np..], depth, "close".to_string());

        np
    }
}

// pub fn parse_many(chars: &[char], depth: usize, vec: &mut Vec<(usize, usize)>, label: String) -> usize {
//     let mut vec = Vec::new();
//     let mut np = 0;
//     while np < chars.len() {
//         parse_rec(&chars[np..], depth, vec, "many".to_string());
//     }
// }

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
            parse_rec(&chars, 0, &mut vec, "root".to_string());
            println!("str: {} parsed: {:?}", str, &vec);
            vec.clear();
        }
        // sample: &str = "[[6,[5,[4,[3,2]]]],1]";

        // parse_rec(&sample.chars().collect_vec(), 0, &mut vec);
        // assert_eq!(vec, vec![(6, 2), (5, 3), (4, 4), (3, 5), (2, 5), (1, 1)])

        // assert_eq!(parse(sample), ((20, 30), (-10, -5)))
    }
}
