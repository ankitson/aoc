use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<isize>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let levels =
        input.lines().map(|ln| ln.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect_vec()).collect_vec();
    levels
}

pub fn part1(raw_input: &str) -> Output {
    let levels = parse(raw_input);
    let mut num_ok = 0;
    for level in levels {
        let mut ok = true;
        let mut prev = level[0];
        let mut incr = true;
        for i in 1..level.len() {
            let curr = level[i];
            if i == 1 && curr < prev {
                incr = false;
            }
            let mut d = curr - prev;
            if !incr && d > 0 || incr && d < 0 {
                ok = false;
                break;
            }
            if !incr {
                d = -d;
            }
            if d < 1 || d > 3 {
                ok = false;
                break;
            }
            prev = curr;
        }
        if ok {
            num_ok += 1
        }
    }
    num_ok
}

pub fn part2(raw_input: &str) -> Output {
    let levels = parse(raw_input);
    // println!("levels: {:?}", levels);
    let mut num_ok = 0;
    for level in levels {
        let mut anyok = false;
        for ri in 0..level.len() {
            let leveln = [&level[0..ri], &level[ri + 1..]].concat();
            let mut ok = true;
            let mut prev = leveln[0];
            let mut incr = true;
            for i in 1..leveln.len() {
                let curr = leveln[i];
                if i == 1 && curr < prev {
                    incr = false;
                }
                let mut d = curr - prev;
                if !incr && d > 0 || incr && d < 0 {
                    ok = false;
                    break;
                }
                if !incr {
                    d = -d;
                }
                if d < 1 || d > 3 {
                    ok = false;
                    break;
                }
                prev = curr;
            }
            anyok = anyok | ok
        }
        if anyok {
            num_ok += 1
        }
        // // let mut ok = true;
        // let mut prev = level[0];
        // let mut incr = true;
        // let mut num_fail = 0;
        // let mut will_skip = false;
        // for i in 1..level.len() {
        //     let curr = level[i];
        //     if i == 1 && curr < prev {
        //         incr = false;
        //     }
        //     let mut d = curr - prev;
        //     if !incr && d > 0 || incr && d < 0 {
        //         num_fail += 1;
        //         will_skip = true
        //     }
        //     if !incr {
        //         d = -d;
        //     }
        //     if d < 1 || d > 3 {
        //         num_fail += 1;
        //         will_skip = true
        //     }
        //     if !will_skip {
        //         prev = curr;
        //     }
        // }
        // if num_fail < 2 {
        //     num_ok += 1
        // }
    }
    num_ok
}
