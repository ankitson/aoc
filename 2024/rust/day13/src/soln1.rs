use itertools::Itertools;
use nalgebra::Matrix2;
use ndarray::{arr1, Array};
use regex::Regex;

pub type NUM = u64;
pub type Input = Vec<((NUM, NUM), (NUM, NUM), (NUM, NUM))>;
pub type Output = NUM;

pub fn parse(input: &str) -> Input {
    let re = Regex::new(r"\d+").unwrap();
    let str_caps = re.captures_iter(input).map(|c| c.get(0).unwrap().as_str().parse().unwrap());
    let chunks = str_caps.chunks(6);
    chunks
        .into_iter()
        .map(|mut chunk| {
            (
                (chunk.next().unwrap(), chunk.next().unwrap()),
                (chunk.next().unwrap(), chunk.next().unwrap()),
                (chunk.next().unwrap(), chunk.next().unwrap()),
            )
        })
        .collect_vec()
}

fn cost(inp: ((NUM, NUM), (NUM, NUM), (NUM, NUM))) -> NUM {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    // solve: A*x = B
    // A = (94 22)  * (x1) = (8400)
    //     (34 67)    (x2)   (5400)
    // x = A^(-1) * B
    let ((ax, ay), (bx, by), (tx, ty)) = inp;
    let mat = Matrix2::new(ax as f64, bx as f64, ay as f64, by as f64);
    let inv = mat.try_inverse().unwrap();
    let result_vec_na = nalgebra::Vector2::new(tx as f64, ty as f64);
    let ans = inv * result_vec_na;
    let a1 = *ans.get(0).unwrap() as NUM;
    let a2 = *ans.get(1).unwrap() as NUM;

    //because we round down
    let a1_cands = vec![a1, a1 + 1];
    let a2_cands = vec![a2, a2 + 1];
    for (a1, a2) in a1_cands.into_iter().cartesian_product(a2_cands) {
        let reached_x = a1 * ax + a2 * bx;
        let reached_y = a1 * ay + a2 * by;
        if reached_x == tx && reached_y == ty {
            return a1 * 3 + a2;
        }
    }
    return 0;
}

pub fn part1(raw_input: &str) -> Output {
    let machines = parse(raw_input);
    machines.into_iter().map(cost).sum()
}

pub fn part2(raw_input: &str) -> Output {
    let machines = parse(raw_input);
    machines
        .into_iter()
        .map(|tup| (tup.0, tup.1, (tup.2 .0 + 10000000000000, tup.2 .1 + 10000000000000)))
        .map(cost)
        .sum()
}
