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
        let mut possible: Vec<usize> = vec![];
        let mut idx = 1;
        for line in lines {
            let gameparts = line.split(':').collect_vec();
            let info = gameparts[1];
            let splits = info.split(';').collect_vec();
            let mut is_valid = true;
            for split in splits {
                let results = split
                    .split(',')
                    .map(|x| x.trim())
                    .map(|x| {
                        let parts = x.split(' ').collect_vec();
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
                let mut cresult = (0, 0, 0);
                for result in results {
                    cresult.0 += result.0;
                    cresult.1 += result.1;
                    cresult.2 += result.2;
                }
                if !(cresult.0 <= 12 && cresult.1 <= 13 && cresult.2 <= 14) {
                    is_valid = false;
                }
            }
            if is_valid {
                possible.push(idx);
            }
            idx += 1;
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
        for line in lines {
            let gameparts = line.split(':').collect_vec();
            let info = gameparts[1];
            let splits = info.split(';').collect_vec();
            let mut cresult = (0, 0, 0);
            for split in splits {
                let results = split
                    .split(',')
                    .map(|x| x.trim())
                    .map(|x| {
                        let parts = x.split(' ').collect_vec();
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
                for result in results {
                    cresult.0 = result.0.max(cresult.0);
                    cresult.1 = result.1.max(cresult.1);
                    cresult.2 = result.2.max(cresult.2);
                }
            }
            power += cresult.0 * cresult.1 * cresult.2;
        }
        power
    }
}
