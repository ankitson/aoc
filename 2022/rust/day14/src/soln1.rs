use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input, false);
        Self::part1_core(input)
    }

    pub fn part1_core(mut input: Input) -> Output {
        let mut sands: usize = 0;
        let mut pos = (0 as isize, 500 as isize);
        while input.get_safe(pos).is_some() {
            let tries = [(pos.0 + 1, pos.1), (pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)];
            if let Some(new_pos) = tries.into_iter().find(|pos| matches!(input.get_safe(*pos), Some(&true))) {
                input.set(pos, true);
                input.set(new_pos, false);
                pos = new_pos;
                continue;
            }

            if tries.into_iter().find(|pos| input.get_safe(*pos).is_none()).is_some() {
                break;
            }

            sands += 1;
            input.set(pos, false);
            pos = (0, 500);
        }
        sands
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input, true);
        Self::part2_core(input)
    }

    pub fn part2_core(mut input: Input) -> Output {
        let mut sands: usize = 0;
        let mut pos = (0 as isize, 500 as isize);
        while input.get_safe(pos).is_some() {
            let tries = [(pos.0 + 1, pos.1), (pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)];
            if let Some(new_pos) = tries.into_iter().find(|pos| matches!(input.get_safe(*pos), Some(&true))) {
                input.set(pos, true);
                input.set(new_pos, false);
                pos = new_pos;
                continue;
            }

            if tries.into_iter().find(|pos| input.get_safe(*pos).is_none()).is_some() {
                break;
            }

            sands += 1;
            input.set(pos, false);

            if !input.get_safe((0, 500)).unwrap() {
                break;
            }
            pos = (0, 500);
        }
        sands
    }
}
