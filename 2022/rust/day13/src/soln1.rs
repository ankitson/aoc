use itertools::Itertools;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};

use crate::shared::{parse, Input, Output};
use nom::bytes::complete::{tag, take_while1};
use nom::{branch::alt, combinator::map, *};
pub struct Soln1 {}
enum Token<'a> {
    Literal(usize),
    List(Vec<Token<'a>>),
    Uneval(&'a str),
}

#[derive(Clone, Debug)]
struct TreeString {
    contents: String,
    pos: Vec<usize>,
}

impl TreeString {
    fn tok(packet: &str) -> IResult<&str, String> {
        alt((nom::character::complete::digit1, tag("["), tag("]"), tag(",")))(packet)
            .map(|res_str| (res_str.0, res_str.1.to_string()))
    }
    fn new(contents: &str) -> TreeString {
        TreeString { contents: contents.to_string(), pos: vec![] }
    }
}

impl Iterator for TreeString {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = TreeString::tok(&self.contents);
            // println!("tok: {x:?}");
            let (rem, tok1) = TreeString::tok(&self.contents).unwrap();
            if rem.is_empty() {
                return None;
            }
            self.contents = rem.to_string();
            let mut in_list = false;
            let mut prev = None;
            match tok1.as_str() {
                "[" => {
                    self.pos.push(0);
                    prev = Some("[");
                    in_list = true;
                }
                "]" => {
                    if in_list {}
                    self.pos.pop();
                }
                "," => {
                    let pos = self.pos.pop().unwrap();
                    let idx = pos + 1;
                    self.pos.push(idx);
                }
                num => {
                    let numv = num.parse::<usize>().unwrap();
                    return Some((numv, self.pos.len(), *self.pos.last().unwrap()));
                }
            }
        }
    }
}

impl Soln1 {
    fn parse(raw_input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_list1(tag("\n\n"), separated_pair(take_while1(|s| s != '\n'), tag("\n"), take_while1(|s| s != '\n')))(
            raw_input,
        )
    }

    pub fn part1(raw_input: &str) -> Output {
        let (rem, mut input) = Self::parse(raw_input).unwrap();
        println!("nom parsed:\n{input:?}");
        let newinp = input.iter_mut().map(|(a, b)| (a.to_string(), b.to_string())).collect_vec();
        Self::part1_core(&newinp)
    }

    pub fn part1_core(input: &Input) -> Output {
        println!("core input: {input:?}");
        let mut orders = vec![];
        for (idx, (left, right)) in input.iter().enumerate() {
            let mut ltree = TreeString::new(left);
            let mut rtree = TreeString::new(right);
            println!("left: {:?}\nright:{:?}", ltree.clone().contents, rtree.clone().contents);

            let mut litem = ltree.next();
            let mut ritem = rtree.next();
            let mut result = true;
            loop {
                println!("\tlval: {litem:?} rval: {ritem:?}");

                if litem.is_none() && ritem.is_none() {
                    break;
                }
                if litem.is_none() && ritem.is_some() {
                    break;
                }
                if litem.is_some() && ritem.is_none() {
                    result = false;
                    break;
                }

                let (lval, ldepth, lidx) = litem.unwrap();
                let (rval, rdepth, ridx) = ritem.unwrap();
                if ldepth == rdepth {
                    // dbg!(lval, rval, ldepth, rdepth);
                    if lval > rval {
                        println!("breaking with false");
                        result = false;
                        break;
                    }
                    litem = ltree.next();
                    ritem = rtree.next();
                } else if ldepth > rdepth {
                    //compare first item from l to r
                    if lval > rval {
                        result = false;
                        break;
                    } else {
                        println!("in skip left");
                        // we cant use skip_while because it takes ownership apparently.
                        // litem = ltree.skip_while(|&x| x.1 > rdepth).next();
                        while let Some((val, depth, idx)) = ltree.next() {
                            // dbg!(depth, rdepth);
                            if depth <= rdepth {
                                litem = Some((val, depth, idx));
                                dbg!(litem);
                                break;
                            }
                        }
                        if litem.unwrap().1 > rdepth {
                            litem = ltree.next();
                        }
                        ritem = rtree.next();
                    }
                } else if ldepth < rdepth {
                    if lval > rval {
                        result = false;
                        break;
                    } else {
                        litem = ltree.next();
                        while let Some((val, depth, idx)) = rtree.next() {
                            if depth <= rdepth {
                                ritem = Some((val, depth, idx));
                                break;
                            }
                        }
                    }
                }
            }
            if result {
                println!("adding idx {}", idx + 1);
                orders.push(idx + 1)
            }
        }

        orders
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
