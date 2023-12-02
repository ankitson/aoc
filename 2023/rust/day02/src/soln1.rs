use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(input: &Input) -> Output {
        let lines = input.lines();
        let mut possible: Vec<usize> = vec![];
        let mut idx = 1;
        for line in lines {
            let gameparts = line.split(":").collect_vec();
            let info = gameparts[1];
            let splits = info.split(";").collect_vec();
            let mut day_results = (0, 0, 0);
            let mut is_valid = true;
            println!("line: {:?}", line);
            for split in splits {
                let results = split
                    .split(",")
                    .map(|x| x.trim())
                    .map(|x| {
                        // println!("line x: {:?}", x);
                        let parts = x.split(" ").collect_vec();
                        // println!("parts: {:?}", parts);
                        let pn = parts[0].parse::<usize>().unwrap();
                        let pc = parts[1];
                        match pc {
                            "red" => {
                                // print!("red = {}", pn);
                                (pn, 0, 0)
                            }
                            "green" => (0, pn, 0),
                            "blue" => (0, 0, pn),
                            _ => panic!("illegal"),
                        }
                    })
                    .collect_vec();
                let mut cresult = (0, 0, 0);
                for result in results {
                    cresult.0 += result.0;
                    cresult.1 += result.1;
                    cresult.2 += result.2;
                }
                if !(cresult.0 <= 12 && cresult.1 <= 13 && cresult.2 <= 14) {
                    is_valid = false;
                }

                // for result in results {
                // day_results.0 += result.0;
                // day_results.1 += result.1;
                // day_results.2 += result.2;
                // }
            }
            println!("Day {}: results = {:?}", idx, day_results);
            if is_valid {
                // if day_results.0 <= 12 && day_results.1 <= 13 && day_results.2 <= 14 {
                println!("Day {} is possible", idx);
                possible.push(idx);
            }
            idx += 1;
            // if idx == 2 {
            // return 0;
            // }
        }
        possible.iter().sum()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let lines = input.lines();
        let mut power: usize = 0;
        let mut idx = 1;
        for line in lines {
            let gameparts = line.split(":").collect_vec();
            let info = gameparts[1];
            let splits = info.split(";").collect_vec();
            let mut cresult = (0, 0, 0);
            for split in splits {
                let results = split
                    .split(",")
                    .map(|x| x.trim())
                    .map(|x| {
                        // println!("line x: {:?}", x);
                        let parts = x.split(" ").collect_vec();
                        // println!("parts: {:?}", parts);
                        let pn = parts[0].parse::<usize>().unwrap();
                        let pc = parts[1];
                        match pc {
                            "red" => (pn, 0, 0),
                            "green" => (0, pn, 0),
                            "blue" => (0, 0, pn),
                            _ => panic!("illegal"),
                        }
                    })
                    .collect_vec();
                // let mut cresult = (0, 0, 0);
                for result in results {
                    // println!("result = {:?}", result);
                    cresult.0 = result.0.max(cresult.0);
                    cresult.1 = result.1.max(cresult.1);
                    cresult.2 = result.2.max(cresult.2);
                }
            }
            // println!("Day {}: results = {:?}", idx, cresult);
            power += (cresult.0 * cresult.1 * cresult.2);
            // if is_valid {
            // if day_results.0 <= 12 && day_results.1 <= 13 && day_results.2 <= 14 {
            // println!("Day {} is possible", idx);
            // possible.push(idx);
            // }
            idx += 1;
            // if idx == 2 {
            // return 0;
            // }
        }
        power
    }
}
