use itertools::Itertools;
use regex::Regex;

pub type P3 = (isize, isize, isize);
pub type Input = Vec<(P3, P3)>;
pub type Output = usize;

pub fn parse(input: &str) -> Vec<(P3, P3)> {
    input
        .lines()
        .map(|line| {
            let (raw_pos, raw_vel) = line.split_once(" @ ").unwrap();
            let (px, py, pz) = raw_pos.split(",").map(|s| s.trim().parse::<isize>().unwrap()).collect_tuple().unwrap();
            let (vx, vy, vz) = raw_vel.split(",").map(|s| s.trim().parse::<isize>().unwrap()).collect_tuple().unwrap();
            ((px, py, pz), (vx, vy, vz))
        })
        .collect_vec()
}

pub fn part1(raw_input: &str, bounds: (P3, P3)) -> Output {
    let stones = parse(raw_input);
    let mut count = 0;
    for comb in stones.iter().combinations(2) {
        let (((p1x, p1y, _), (v1x, v1y, _)), ((p2x, p2y, _), (v2x, v2y, _))) = (comb[0], comb[1]);
        let m1 = *v1y as f64 / *v1x as f64;
        let m2 = *v2y as f64 / *v2x as f64;
        let c1 = *p1y as f64 - m1 * *p1x as f64;
        let c2 = *p2y as f64 - m2 * *p2x as f64;

        if m1 == m2 {
            continue;
        }
        let x_intsct = ((c2 - c1) / (m1 - m2)) as isize;
        let y_instct = (m1 * x_intsct as f64 + c1) as isize;

        let in_bounds =
            x_intsct >= bounds.0 .0 && x_intsct <= bounds.1 .0 && y_instct >= bounds.0 .1 && y_instct <= bounds.1 .1;
        let direction = (*v1x >= 0 && x_intsct >= *p1x || *v1x < 0 && x_intsct <= *p1x)
            && (*v2x >= 0 && x_intsct >= *p2x || *v2x < 0 && x_intsct <= *p2x);
        if in_bounds && direction {
            count += 1;
        }
    }
    count
}

pub fn part2(raw_input: &str) -> Output {
    let stones = parse(raw_input);

    todo!()
}
