use itertools::Itertools;
use regex::Regex;

pub type Input = String;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.to_string()
    // input.lines().map(|ln| ln.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec()).collect_vec();
    // todo!()
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&input);
    let mut prod = 0;
    for matc in matches {
        let (_, [n1s, n2s]) = matc.extract();
        let n1: usize = n1s.parse().unwrap();
        let n2: usize = n2s.parse().unwrap();
        prod += n1 * n2
    }
    prod
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re.captures_iter(&input);
    let re2 = Regex::new(r"(don't|do)").unwrap();
    let dodonts = re2.find_iter(&input).map(|m| (m.as_str(), m.end())).collect_vec();
    let mut prod = 0;
    for matc in matches {
        let start = matc.get(0).unwrap().start();
        let (_, [n1s, n2s]) = matc.extract();
        let n1: usize = n1s.parse().unwrap();
        let n2: usize = n2s.parse().unwrap();
        // println!("dodonts: {:?}", dodonts);
        let mut prev_str = "do";
        for (mstr, mend) in &dodonts {
            if *mend > start {
                break;
            }
            prev_str = mstr
        }
        if prev_str == "do" {
            prod += n1 * n2
        }
    }
    prod

    // todo!()
}
