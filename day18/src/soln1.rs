use std::ops::Div;

use crate::shared::parse;
use itertools::iproduct;

type FN = Vec<(usize, usize)>;
pub struct Soln1 {}
impl Soln1 {
    pub fn split(fishnum: &FN) -> Option<FN> {
        let mut next: FN = vec![];
        for i in 0..fishnum.len() {
            let (num, depth) = fishnum[i];
            if num > 10 {
                next.extend_from_slice(&fishnum[0..i]);
                next.push((num.unstable_div_floor(2), depth + 1));
                next.push((num.unstable_div_ceil(2), depth + 1));
                if i + 1 < fishnum.len() {
                    next.extend_from_slice(&fishnum[i + 1..]);
                }
                return Some(next);
            }
        }
        None
    }

    pub fn explode(fishnum: &FN) -> Option<FN> {
        let mut next: FN = vec![];
        let mut reduced: FN = vec![];
        for i in 0..fishnum.len() - 1 {
            let (numl, depthl) = fishnum[i];
            let (numr, depthr) = fishnum[i + 1];
            if depthl >= 4 && depthr == depthl {
                //pair of adjacent elems at equal depth represents a leaf fishnum
                next = fishnum.clone();
                if i > 0 {
                    next[i - 1].0 += numl;
                }
                if i + 2 < fishnum.len() {
                    next[i + 2].0 += numr;
                }
                let (lef, mut rig) = next.split_at(i);
                rig = &rig[2..];

                reduced.extend_from_slice(lef);
                reduced.push((0, depthl - 1));
                reduced.extend_from_slice(rig);
                return Some(reduced);
            }
        }
        None
    }

    pub fn reduce(fishnum: &FN) -> FN {
        if let Some(next) = Self::explode(fishnum) {
            next
        } else if let Some(next) = Self::split(fishnum) {
            next
        } else {
            fishnum.clone()
        }
    }

    pub fn reduce_full(mut fishnum: FN) -> FN {
        let mut reduced = Self::reduce(&fishnum);
        while fishnum != reduced {
            fishnum = reduced;
            reduced = Self::reduce(&fishnum);
        }
        reduced
    }

    pub fn magnitude_step(fishnum: &FN) -> FN {
        if fishnum.len() == 1 {
            return fishnum.to_vec();
        }
        let mut next: FN = vec![];
        for i in 0..fishnum.len() - 1 {
            let (numl, depthl) = fishnum[i];
            let (numr, depthr) = fishnum[i + 1];
            if depthl == depthr {
                //pair of adjacent elems at equal depth represents a leaf fishnum
                //next = fishnum.clone();
                let magn = numl * 3 + numr * 2;
                let (l, mut r) = fishnum.split_at(i);
                next.extend_from_slice(l);
                next.push((magn, depthl - 1));
                next.extend_from_slice(&r[2..]);
                return next;
            }
        }
        panic!("cant magnitude step this num: {:?}", fishnum);
    }

    pub fn magnitude(fishnum: FN) -> usize {
        let mut fncopy = fishnum.clone();
        while fncopy.len() > 1 {
            fncopy = Self::magnitude_step(&fncopy);
        }
        fncopy[0].0
    }

    pub fn add(l: &mut FN, r: &FN) -> () {
        l.extend(r);
        for (n, d) in l {
            *d += 1;
        }

        //l.iter_mut().map(|(n, d)| *d += 1);
    }

    pub fn sum_many(input: Vec<FN>) -> FN {
        let mut accum = input[0].clone();
        for num in &input[1..] {
            Self::add(&mut accum, num)
        }
        accum
    }

    pub fn part1(input: &str) -> usize {
        let mut parsed = parse(input);
        let mut sum = Self::sum_many(parsed);
        println!("Sum: {:?}", sum);
        let mut reduced = Self::reduce_full(sum);
        println!("Reduced: {:?}", reduced);
        // let mut magnstep = Self::magnitude_step(&reduced);
        // println!("Magnitude step: {:?}", magnstep);
        // 0
        Self::magnitude(reduced)
    }

    pub fn part2(input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::identity_op)]
    use super::{Soln1, FN};
    use crate::shared::{parse, parse_one};

    #[test]
    fn test_magnitude() {
        let inouts: Vec<(FN, usize)> = vec![
            (parse_one("[9,1]"), 29),
            (parse_one("[[9,1],[1,9]]"), 129),
            (parse_one("[[1,2],[[3,4],5]]"), 143),
            (parse_one("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), 3488),
        ];
        for (inp, out) in inouts {
            assert_eq!(Soln1::magnitude(inp), out)
        }
    }

    #[test]
    fn test_sum() {
        let mut n1 = parse_one("[1,1]");
        let n2 = parse_one("[2,2]");
        Soln1::add(&mut n1, &n2);
        assert_eq!(n1, vec![(1, 2), (1, 2), (2, 2), (2, 2)]);

        let str = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
        let parsed = parse(&str);
        println!("parsed: {:?}", parsed);
        let sum = Soln1::sum_many(parsed);
        println!("sum: {:?}", sum);
    }

    #[test]
    fn test_reduce() {
        let inouts: Vec<(FN, FN)> = vec![(parse_one("[[[[[9,8],1],2],3],4]"), parse_one("[[[[0,9],2],3],4]"))];
        for (inp, out) in &inouts {
            println!("Input: {:?}\nExpected Out: {:?}", inp, out);
            assert_eq!(&Soln1::reduce(inp), out)
        }
    }

    #[test]
    fn test_part1() {
        let sample2: &str = include_str!("../inputs/sample2.txt");
        let part1 = Soln1::part1(sample2);
        println!("Part 1 (sample) = {:?}", part1);
        // assert_eq!(part1, 45);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = Soln1::part2(sample);
        // println!("Part 2 = {:?}", part2);
        // assert_eq!(part2, 112);
    }
}
