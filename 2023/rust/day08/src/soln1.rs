use std::collections::HashMap;

use itertools::Itertools;

pub type Input = (Vec<char>, HashMap<&'static str, (&'static str, &'static str)>);
pub type Output = usize;

pub fn parse(input: &'static str) -> Input {
    let mut lines = input.lines();
    let instrs = lines.next().unwrap().chars().collect_vec();
    lines.next();

    let mut nodes = HashMap::new();
    for line in lines {
        let split = line.split_once(" = ").unwrap();
        let node = split.0;
        let left = &split.1[1..=3];
        let right = &split.1[6..=8];
        nodes.insert(node, (left, right));
    }
    (instrs, nodes)
}

pub fn part1(raw_input: &'static str) -> Output {
    let (instrs, nodes) = parse(raw_input);
    let mut curr = "AAA";
    let mut iptr = 0;
    while curr != "ZZZ" {
        match instrs[iptr % instrs.len()] {
            'L' => curr = nodes.get(&curr).unwrap().0,
            'R' => curr = nodes.get(&curr).unwrap().1,
            _ => panic!("illegal inst"),
        }
        iptr += 1;
    }
    iptr
}

pub fn part2(raw_input: &'static str) -> Output {
    let (instrs, nodes) = parse(raw_input);
    let starts = nodes.keys().filter(|k| k.chars().last().unwrap() == 'A').collect_vec();

    let mut cycle_lens = HashMap::new();
    for &start in starts {
        let mut iptr = 0;
        let mut curr = start;
        while curr.chars().last().unwrap() != 'Z' {
            match instrs[iptr % instrs.len()] {
                'L' => curr = nodes.get(&curr).unwrap().0,
                'R' => curr = nodes.get(&curr).unwrap().1,
                _ => panic!("illegal inst"),
            }
            iptr += 1;
        }
        cycle_lens.insert(curr, iptr);
    }

    fn lcm(a: usize, b: usize) -> usize {
        let smaller = a.min(b);
        let bigger = a.max(b);
        let mut curr = bigger;
        while !(curr % smaller == 0) {
            curr += bigger;
        }
        curr
    }

    let lcm = cycle_lens.values().fold(1, |acc, cl| lcm(acc, *cl));
    lcm
}
