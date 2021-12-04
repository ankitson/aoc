/*
  00100
  11000
  11111
-------
  11100 most common
  00011 least common

mc = NOT lc

mc * lc = mc * NOT mc

1 * 0 = 0

00 * 11 = 0b
01 * 10 = 10b
10 * 01 = 10b
11 * 00 = 0b

001 * 110 = 1 * 6 = 6 = 100b
011 * 100 = 3 * 4 = 12 = 100b
010 * 101 = 2 * 5 = 10 =
001 * 110

*/

fn input() -> impl Iterator<Item = u32> {
    let contents: &str = include_str!("../../input/day3.txt");
    contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| u32::from_str_radix(x, 2).expect("illegal int ahah"))
}

fn widen_mul(a: u16, b: u16) -> u32 {
    let mut result: u32 = 0;
    let mut ac = a;
    while ac > 0 {
        result += b as u32;
        ac -= 1;
    }
    result
}

pub fn part1() -> String {
    let nums = input().collect::<Vec<u32>>();

    let mut gamma: u16 = 0; //most common
    for bi in 0..12 {
        let mut c1 = 0;
        let mut c0 = 0;
        for num in &nums {
            let bit = (num >> bi) & 1;
            if bit == 1 {
                c1 += 1
            } else {
                c0 += 1
            }
        }
        let most_common = {
            if c1 > c0 {
                1
            } else if c0 > c1 {
                0
            } else {
                panic!("equal 0s and 1s at index {}", bi);
            }
        };
        gamma |= most_common << bi;
    }
    let mask = 0b111111111111;
    let epsilon = (!gamma) & mask;
    println!("gamma = {} epsilon = {}", gamma, epsilon);
    let prod = widen_mul(gamma, epsilon);
    format!("{}", prod)
}

pub fn part2() -> String {
    "bla".to_string()
}
