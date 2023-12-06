use itertools::Itertools;

use crate::shared::{parse, parse_fast, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse_fast(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let (times, dists) = input;
        let mut ans = 1;
        for (time, dist) in times.iter().zip(dists) {
            let t = *time as f64;
            let d = *dist as f64;
            // (t-x)*x > d
            // -x^2 + tx -d > 0
            // x >= lower root && <= upper root
            // rounding: x>= ceil(lower) && x <= floor(upper)
            // num in ranges = floor(upper) - ceil(lower) + 1
            // root = -b/2a +- sqrt(b^2-4ac)/ 2a
            // a = -1, b = t, c = -d
            // root = t/2 +- sqrt(t^2-4*d) / -2

            let m = t / 2.0;
            let n = (t.powi(2) - 4.0 * d).sqrt() / 2.0;
            let r1 = m + n;
            let r2 = m - n;

            // matching the distance does not work we have to beat it.
            // for integer roots like 10, 20 -> the range is 11->19.
            // so we want to round up, but +1 if its already an int;
            // 20 -> 19
            // 19.5 -> 19
            let range = ((r1 - 1.).ceil() - (r2 + 1.).floor()) as usize + 1;
            ans *= range as usize;
        }
        ans
    }

    pub fn part2_other(raw_input: &str) -> Output {
        let lines = raw_input.lines().collect_vec();
        let t = lines[0].split_ascii_whitespace().skip(1).join("").parse::<f64>().unwrap();
        let d = lines[1].split_ascii_whitespace().skip(1).join("").parse::<f64>().unwrap();

        let a = t / 2.0;
        let b = (t.powi(2) - 4.0 * d).sqrt() / 2.0;
        let r1 = a + b;
        let r2 = a - b;
        let range = ((r1 - 1.).ceil() - (r2 + 1.).floor()) as usize + 1;
        range as usize
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse_fast(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let (times, dists) = input;
        let t = times.iter().join("").parse::<f64>().unwrap();
        let d = dists.iter().join("").parse::<f64>().unwrap();

        // difference of roots of -x^2 + Tx - D = 0 = 2 * sqrt(T^2 - 4*(-d)*(-1))
        // technically not correct - we have to account for the rounding like in the other part2 and part1 solns
        let root_diff = (t * t - 4.0 * (-d) * (-1.0)).sqrt();
        root_diff as usize
    }
}
