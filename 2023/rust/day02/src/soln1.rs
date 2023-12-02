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
        let mut possible_sum = 0;
        let mut idx = 1;

        for line in lines {
            let mut is_valid = true;

            let (_, info) = line.split_once(": ").unwrap();
            let sets = info.split([';', ',']);
            for set in sets {
                let (pn, pc) = set.trim().split_once(' ').unwrap();
                let pn = pn.parse::<usize>().unwrap();
                match pc.as_bytes()[0] {
                    b'r' => is_valid &= pn <= 12,
                    b'g' => is_valid &= pn <= 13,
                    b'b' => is_valid &= pn <= 14,
                    _ => panic!("illegal"),
                }
            }
            if is_valid {
                possible_sum += idx;
            }
            idx += 1;
        }
        possible_sum
    }

    pub fn part1_parsing(raw_input: &str) -> Output {
        #[inline(always)]
        fn expect_any(str: &str, n: usize) -> &str {
            &str[n..]
        }
        #[inline(always)]
        fn expect_game(str: &str) -> &str {
            &str["Game ".len()..]
        }
        #[inline(always)]
        fn expect_n(str: &str) -> &str {
            &str[str.find(":").unwrap() + 2..]
        }
        #[inline(always)]
        fn accept_n(str: &str) -> (&str, usize) {
            let mut chars = str.chars();
            let cd0 = chars.next().unwrap();
            let pd1 = chars.next().unwrap();
            if pd1.is_ascii_digit() {
                let d1 = (pd1 as u8) - ('0' as u8);
                let d0 = (cd0 as u8) - ('0' as u8);
                let n = 10 * d0 + d1;
                return (&str[2..], n as usize);
            } else {
                return (&str[1..], (cd0 as usize) - ('0' as usize));
            }
        }
        fn accept_color(str: &str) -> (&str, u8) {
            //first is space
            match str.as_bytes()[1] {
                b'r' => (&str[4..], 0),
                b'g' => (&str[6..], 1),
                b'b' => (&str[5..], 2),
                _ => panic!("expected color"),
            }
        }

        let lines = raw_input.lines();
        let mut possible_sum = 0;
        let mut idx = 1;
        for line in lines {
            let mut is_valid = true;

            let mut input = line;
            input = expect_game(input);
            input = expect_n(input);
            while input.len() > 0 {
                let (ninput, n) = accept_n(input);
                input = ninput;
                let (ninput, c) = accept_color(input);
                input = ninput;
                if (n > 12 && c == 0) || (n > 13 && c == 1) || (n > 14 && c == 2) {
                    is_valid = false;
                    break;
                }
                if input.len() == 0 {
                    break;
                }

                let ninput = expect_any(input, 2);
                input = ninput;
            }
            if is_valid {
                possible_sum += idx;
            }

            idx += 1;
        }

        possible_sum
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        let lines = input.lines();
        let mut power: usize = 0;
        for line in lines {
            let mut gameparts = line.split(':');
            let info = gameparts.nth(1).unwrap();
            let splits = info.split(';');
            let mut cresult = (0, 0, 0);
            for split in splits {
                split.split(',').for_each(|x| {
                    let mut parts = x.trim().split(' ');
                    let pn = parts.next().unwrap().parse::<usize>().unwrap();
                    let pc = parts.next().unwrap();
                    match pc {
                        "red" => cresult.0 = pn.max(cresult.0),
                        "green" => cresult.1 = pn.max(cresult.1),
                        "blue" => cresult.2 = pn.max(cresult.2),
                        _ => panic!("illegal"),
                    }
                })
            }
            power += cresult.0 * cresult.1 * cresult.2;
        }
        power
    }

    pub fn part2_parsing(raw_input: &str) -> Output {
        #[inline(always)]
        fn expect_any(str: &str, n: usize) -> &str {
            &str[n..]
        }
        #[inline(always)]
        fn expect_game(str: &str) -> &str {
            &str["Game ".len()..]
        }
        #[inline(always)]
        fn expect_n(str: &str) -> &str {
            &str[str.find(":").unwrap() + 2..]
        }
        #[inline(always)]
        fn accept_n(str: &str) -> (&str, usize) {
            let mut chars = str.chars();
            let cd0 = chars.next().unwrap();
            let pd1 = chars.next().unwrap();
            if pd1.is_ascii_digit() {
                let d1 = (pd1 as u8) - ('0' as u8);
                let d0 = (cd0 as u8) - ('0' as u8);
                let n = 10 * d0 + d1;
                return (&str[2..], n as usize);
            } else {
                return (&str[1..], (cd0 as usize) - ('0' as usize));
            }
        }
        fn accept_color(str: &str) -> (&str, u8) {
            //first is space
            match str.as_bytes()[1] {
                b'r' => (&str[4..], 0),
                b'g' => (&str[6..], 1),
                b'b' => (&str[5..], 2),
                _ => panic!("expected color"),
            }
        }

        let lines = raw_input.lines();

        let mut power = 0;
        for line in lines {
            let mut maxr = 0;
            let mut maxg = 0;
            let mut maxb = 0;
            let mut input = line;
            input = expect_game(input);
            input = expect_n(input);
            while input.len() > 0 {
                let (ninput, n) = accept_n(input);
                input = ninput;
                let (ninput, c) = accept_color(input);
                input = ninput;
                if c == 0 {
                    maxr = maxr.max(n);
                } else if c == 1 {
                    maxg = maxg.max(n);
                } else if c == 2 {
                    maxb = maxb.max(n);
                } else {
                    unreachable!();
                }
                if input.len() == 0 {
                    break;
                }

                let ninput = expect_any(input, 2);
                input = ninput;
            }
            power += maxr * maxg * maxb;
        }

        power
    }
}
