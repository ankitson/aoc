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
        print!("stones: {:?}, {:?}, x_intsct: {} y_instct: {} ", comb[0], comb[1], x_intsct, y_instct);
        if x_intsct >= bounds.0 .0
            && x_intsct <= bounds.1 .0
            && y_instct >= bounds.0 .1
            && y_instct <= bounds.1 .1
            && (*v1x >= 0 && x_intsct >= *p1x || *v1x < 0 && x_intsct <= *p1x)
            && (*v2x >= 0 && x_intsct >= *p2x || *v2x < 0 && x_intsct <= *p2x)
        {
            println!("\t +1");
            count += 1;
        } else {
            println!("");
        }

        // if x_intsct >= min_x && x_intsct <= max_x {
        // count += 1;
        // }
    }
    count
    //p1x_t = p1x_0 + v_1x*t
    //p2x_t = p2x_0 + v_2x*t
    //p1x_t = p2x_t
    //p1x_0 + v_1x*A = p2x_0 + v_2x*B (they can cross at different times)

    // pos = 0 , 0
    // vel = 15, 20
    // p1 = 15, 20
    // p2 = 30, 40..

    //px lies on line y = vy*x + px_0
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
