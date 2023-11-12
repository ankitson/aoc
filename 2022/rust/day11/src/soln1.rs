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
        let mut monkeInspects = HashMap::<usize, usize>::new();
        for _ in 0..20 {
            for i in 0..monkes.len() {
                let monke = &mut monkes[i];
                let op = &monke.op;
                let len = monke.items.len();
                while len > 0 {
                    monkeInspects.entry(i).and_modify(|x| *x += 1).or_insert(1);
                    let item = monke.items.pop_front().unwrap();
                    let new_val = op.eval(item) / 3;
                    let dest = match new_val % monke.divisor {
                        0 => monke.throw_true,
                        _ => monke.throw_false,
                    };
                    let mut dest_monke = monkes[dest];
                    dest_monke.items.push_back(new_val);
                }
            }
        }
        let most = monkeInspects.values().sorted_by(|a, b| Ord::cmp(&b, &a)).take(2).collect_vec();
        let monke_bizness = most[0] * most[1];
        monke_bizness
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
