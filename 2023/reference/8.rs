// From magnus hegdahl on AOC channel in Rust discord
use aoc_zen_runner_macros::aoc;

#[aoc(2023, day8)]
pub mod solutions {
    use aoc_zen_runner_macros::solution;

    fn solve(mut current: u16, lr: &[u8], mapping: &[[u16; 2]]) -> u64 {
        let mut steps = 0u64;
        while current < 25 * 26 * 26 {
            let side = lr[(steps % (lr.len() as u64)) as usize];
            current = mapping[current as usize][side as usize];

            steps += 1;
            if steps == 100_000_000 {
                break;
            }
        }

        steps
    }

    fn parse_input(input: &str) -> (Vec<u8>, Vec<[u16; 2]>) {
        let mut lines = input.lines();
        let lr = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            })
            .collect();

        lines.next(); // empty line

        let mut mapping = vec![[u16::MAX; 2]; 26 * 26 * 26];

        for line in lines {
            let line = line.as_bytes();

            let id = |a: &[u8]| {
                let mut res = 0u16;
                for &x in a.iter().rev() {
                    res *= 26;
                    res += (x - b'A') as u16;
                }
                res
            };

            let x = id(&line[0..3]);
            let l = id(&line[7..10]);
            let r = id(&line[12..15]);
            mapping[x as usize] = [l, r];
        }

        (lr, mapping)
    }

    #[solution(part1, brute)]
    pub fn part1(input: &str) -> u64 {
        let (lr, mapping) = parse_input(input);
        solve(0, &lr, &mapping)
    }

    fn gcd(mut m: u64, mut n: u64) -> u64 {
        if m == 0 || n == 0 {
            return m | n;
        }

        let shift = (m | n).trailing_zeros();

        n >>= n.trailing_zeros();

        while m != 0 {
            m >>= m.trailing_zeros();
            if n > m {
                std::mem::swap(&mut n, &mut m)
            }
            m -= n;
        }

        n << shift
    }

    fn lcm(m: u64, n: u64) -> u64 {
        n * m / gcd(m, n)
    }

    #[solution(part2, brute)]
    pub fn part2(input: &str) -> u64 {
        let (lr, mapping) = parse_input(input);

        let mut period = 1;
        for i in 0..26 * 26 {
            if mapping[i as usize][0] != u16::MAX {
                period = lcm(period, solve(i, &lr, &mapping));
            }
        }

        period
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;

    #[allow(non_upper_case_globals)]
    #[aoc_case(2, 2)]
    const input1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    #[allow(non_upper_case_globals)]
    #[aoc_case(6, 6)]
    const input2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
}
