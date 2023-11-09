use itertools::Itertools;
use regex::Regex;
use scan_fmt::scan_fmt;

pub type Input<'a> = Vec<(&'a str, u32, Vec<String>)>;
pub type Output = String;

pub fn parse<'a>(input: &'a str) -> Input {
    //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

    let mut out = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once(";").expect("illegal line");
        let mut s = a.split_ascii_whitespace().collect_vec();
        // dbg!(&s);
        let valve = s[1];
        let rate = s[4].split_once("=").expect("illegal").1.parse::<u32>().expect("illegal");
        // dbg!(&rate);
        // dbg!(&b);

        let tunnels = b
            .split("valve")
            .nth(1)
            .map(|c| {
                // dbg!(c);
                c
            })
            .expect("illegal")
            .split(", ")
            .map(|c| {
                if (c.chars().nth(0).expect("a") == 's') {
                    c.chars().skip(2).collect::<String>()
                } else {
                    c.to_string()
                }
            })
            .map(|c| c.trim().to_string())
            .collect_vec();
        out.push((valve, rate, tunnels));
    }
    out
}
