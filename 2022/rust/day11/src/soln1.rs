use std::collections::HashMap;

use itertools::Itertools;

use crate::shared::{parse, Input, Monke, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let mut input = parse(raw_input);
        Self::part1_core(&mut input)
    }

    pub fn part1_core(input: &mut Input) -> Output {
        let monkes: &mut Vec<Monke> = input;
        let mut monke_inspects = HashMap::<usize, usize>::new();
        for _round in 0..20 {
            for i in 0..monkes.len() {
                let monke = &monkes[i];
                let monke_clone = &mut monke.clone();
                monke_inspects.entry(i).and_modify(|v| *v += monke.items.len()).or_insert(monke.items.len());

                let op = monke_clone.op;
                while monke_clone.items.len() > 0 {
                    let item = monke_clone.items.pop_front().unwrap();
                    let new_val = op.eval(item) / 3;
                    let dest = match new_val % monke_clone.divisor {
                        0 => monke_clone.throw_true,
                        _ => monke_clone.throw_false,
                    };
                    let dest_monke = &mut monkes[dest];
                    dest_monke.items.push_back(new_val);
                }
                monkes[i] = monke_clone.clone(); //why another clone here?
            }
        }
        let most = monke_inspects.values().sorted_by(|a, b| Ord::cmp(&b, &a)).take(2).collect_vec();
        let monke_bizness = most[0] * most[1];
        monke_bizness
    }

    pub fn part2(raw_input: &str) -> Output {
        let mut input = parse(raw_input);
        Self::part2_core(&mut input)
    }

    pub fn part2_core(input: &mut Input) -> Output {
        let monkes: &mut Vec<Monke> = input;
        let mut monke_inspects = HashMap::<usize, usize>::new();
        let divisor_prod: usize = monkes.iter().map(|m| m.divisor).product();
        for _round in 0..10000 {
            for i in 0..monkes.len() {
                let monke = &monkes[i];
                let monke_clone = &mut monke.clone();
                monke_inspects.entry(i).and_modify(|v| *v += monke.items.len()).or_insert(monke.items.len());

                let op = monke_clone.op;
                while monke_clone.items.len() > 0 {
                    let item = monke_clone.items.pop_front().unwrap();
                    let item = item % divisor_prod;
                    let new_val = op.eval(item);
                    let dest = match new_val % monke_clone.divisor {
                        0 => monke_clone.throw_true,
                        _ => monke_clone.throw_false,
                    };
                    let dest_monke = &mut monkes[dest];
                    dest_monke.items.push_back(new_val);
                }
                monkes[i] = monke_clone.clone(); //why another clone here?
            }
        }
        let most = monke_inspects.values().sorted_by(|a, b| Ord::cmp(&b, &a)).take(2).collect_vec();
        let monke_bizness = most[0] * most[1];
        monke_bizness
    }
}
