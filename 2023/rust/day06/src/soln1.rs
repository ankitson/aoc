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
            let a = t / 2.0;
            let b = (t.powi(2) - 4.0 * d).sqrt() / 2.0;
            let r1 = a + b;
            let r2 = a - b;

            // matching the distance does not work we have to beat it.
            // for integer roots like 10, 20 -> the range is 11->19.
            // so we want to round up, but +1 if its already an int
            let lower = if r2.fract() == 0.0 { r2 as usize + 1 } else { r2.ceil() as usize };
            let upper = if r1.fract() == 0.0 { r1 as usize - 1 } else { r1.floor() as usize };

            let range = upper - lower + 1;
            ans *= range as usize;
        }
        ans
    }

    pub fn part2_fast(raw_input: &str) -> Output {
        let lines = raw_input.lines().collect_vec();
        let t = lines[0].split_ascii_whitespace().skip(1).join("").parse::<f64>().unwrap();
        let d = lines[1].split_ascii_whitespace().skip(1).join("").parse::<f64>().unwrap();
        let root_diff = (t * t - 4.0 * (-d) * (-1.0)).sqrt();
        root_diff as usize
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
