use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let (times, dists) = input;
        let mut ans = 1;
        for (time, dist) in times.iter().zip(dists) {
            ans *= (0..*time).fold(0, |wins, hold| {
                let dist_covered = (time - hold) * hold;
                if dist_covered > *dist {
                    wins + 1
                } else {
                    wins
                }
            });
        }
        ans
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let (times, dists) = input;
        let t = times.iter().join("").parse::<f64>().unwrap();
        let d = dists.iter().join("").parse::<f64>().unwrap();
        // difference of roots of -x^2 + Tx - D = 0 = 2 * sqrt(T^2 - 4*(-d)*(-1)) /
        let root_diff = (t * t - 4.0 * (-d) * (-1.0)).sqrt();
        root_diff as usize
    }
}
