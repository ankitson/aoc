use crate::shared::parse;
use itertools::iproduct;

type FN = Vec<(usize, usize)>;
pub struct Soln1 {}
impl Soln1 {
    pub fn split(fishnum: &FN) -> Option<FN> {
        let mut next: FN = vec![];
        for i in 0..fishnum.len() {
            let (num, depth) = fishnum[i];
            if num >= 10 {
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
        let mut reduced: FN = vec![];
        for i in 0..fishnum.len() - 1 {
            let (numl, depthl) = fishnum[i];
            let (numr, depthr) = fishnum[i + 1];
            if depthl >= 5 && depthr == depthl {
                // println!("exploding {} {} at depth {}", numl, numr, depthl);
                //pair of adjacent elems at equal depth represents a leaf fishnum
                let mut next = fishnum.clone();
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
            // println!("Reduce {} = {}", Self::fmt_num(&fishnum), Self::fmt_num(&reduced));
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
        let mut fncopy = fishnum;
        while fncopy.len() > 1 {
            fncopy = Self::magnitude_step(&fncopy);
        }
        fncopy[0].0
    }

    pub fn add(l: &mut FN, r: &FN) {
        l.extend(r);
        for (n, d) in l {
            *d += 1;
        }
    }

    pub fn part1(input: &str) -> usize {
        let mut parsed = parse(input);
        let mut accum = parsed[0].clone();
        for num in &parsed[1..] {
            let lhs = accum.clone();
            Self::add(&mut accum, num);
            let prered = accum.clone();
            accum = Self::reduce_full(accum);
            // println!(
            //     "Step: {} + {} = {}",
            //     Self::fmt_num(&lhs),
            //     Self::fmt_num(num),
            //     Self::fmt_num(&accum)
            // );
        }
        Self::magnitude(accum)
    }

    pub fn fmt_num(num: &FN) -> String {
        let mut chars = Vec::new();
        let mut br = 0;
        for i in 0..num.len() {
            let (n, d) = num[i];
            let mut closed = false;
            while br < d {
                br += 1;
                chars.push('[');
            }
            while br > d {
                br -= 1;
                chars.push(']');
                closed = true;
            }
            if closed {
                chars.push(',');
            }

            chars.extend(n.to_string().chars());
            if i != num.len() - 1 && num[i + 1].1 >= d {
                chars.push(',')
            }
        }
        while br > 0 {
            br -= 1;
            chars.push(']');
        }

        chars.iter().collect()
    }

    pub fn part2(input: &str) -> usize {
        let mut inputs = parse(input);
        let ninputs = inputs.len();

        let mut max_magnitude = usize::MIN;
        for (i1, i2) in iproduct![0..ninputs, 0..ninputs] {
            if i1 == i2 {
                continue;
            }
            let lhs = &inputs[i1];
            let rhs = &inputs[i2];
            let mut accum = lhs.clone();
            Self::add(&mut accum, rhs);
            let reduced = Self::reduce_full(accum);
            let mut magnitude = Self::magnitude(reduced);
            max_magnitude = max_magnitude.max(magnitude);
        }
        max_magnitude
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::identity_op)]
    use itertools::Itertools;

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
        let n1 = parse_one("[1,1]");
        let n2 = parse_one("[2,2]");
        let mut accum = n1.clone();
        Soln1::add(&mut accum, &n2);
        assert_eq!(accum, vec![(1, 2), (1, 2), (2, 2), (2, 2)]);
        println!(
            "{} + {} = {}",
            Soln1::fmt_num(&n1),
            Soln1::fmt_num(&n2),
            Soln1::fmt_num(&accum)
        );
    }

    #[test]
    fn test_reduce() {
        let pairs = (vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
            //split
            ("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
            //explode
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
            //explode
            (
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
        ]);
        let inouts: Vec<(FN, FN)> = Vec::new();
        for (instr, outstr) in pairs {
            println!("Input: {}", instr);
            let parse = parse_one(instr);
            println!("Parse: {:?}", parse);
            let reduce = Soln1::reduce(&parse);
            println!("Reduce1: {:?}", reduce);

            println!("Expected: {}", outstr);
            let parse_out = parse_one(outstr);
            assert_eq!(reduce, parse_out);
        }
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = Soln1::part1(sample);
        println!("Part 1 (sample1) = {:?}", part1);

        let sample: &str = include_str!("../inputs/sample2.txt");
        let part1 = Soln1::part1(sample);
        println!("Part 1 (sample2) = {:?}", part1);

        assert_eq!(part1, 4140);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample2.txt");
        let part2 = Soln1::part2(sample);
        println!("Part 2 (sample2) = {:?}", part2);
        assert_eq!(part2, 3993);
    }
}
