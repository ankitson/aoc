use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let lines = input.lines();
        let mut possible_sum = 0;
        let mut idx = 1;
        for line in lines {
            let mut is_valid = true;

            let (_, info) = line.split_once(": ").unwrap();
            let sets = info.split(';');
            for split in sets {
                split.split(',').for_each(|x| {
                    let (pn, pc) = x.trim().split_once(' ').unwrap();
                    let pn = pn.parse::<usize>().unwrap();
                    match pc.as_bytes()[0] {
                        b'r' => is_valid &= pn <= 12,
                        b'g' => is_valid &= pn <= 13,
                        b'b' => is_valid &= pn <= 14,
                        _ => panic!("illegal"),
                    }
                })
            }
            if is_valid {
                possible_sum += idx;
            }
            idx += 1;
        }
        possible_sum
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let lines = input.lines();
        let mut power: usize = 0;
        for line in lines {
            let mut gameparts = line.split(':');
            let info = gameparts.nth(1).unwrap();
            let splits = info.split(';');
            let mut cresult = (0, 0, 0);
            for split in splits {
                split.split(',').for_each(|x| {
                    let mut parts = x.trim().split(' ');
                    let pn = parts.next().unwrap().parse::<usize>().unwrap();
                    let pc = parts.next().unwrap();
                    match pc {
                        "red" => cresult.0 = pn.max(cresult.0),
                        "green" => cresult.1 = pn.max(cresult.1),
                        "blue" => cresult.2 = pn.max(cresult.2),
                        _ => panic!("illegal"),
                    }
                })
            }
            power += cresult.0 * cresult.1 * cresult.2;
        }
        power
    }
}
