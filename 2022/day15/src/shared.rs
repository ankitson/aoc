use itertools::Itertools;
use regex::Regex;
use scan_fmt::scan_fmt;

pub type Input = Vec<((i32, i32), (i32, i32))>;
pub type Output = i32;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (sx, sy, cx, cy) =
                scan_fmt!(line, "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}", i32, i32, i32, i32)
                    .expect("illegal input");
            ((sx, sy), (cx, cy))
        })
        .collect_vec()
}
