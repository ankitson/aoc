use crate::shared::parse;
use itertools::{iproduct, Itertools};

pub struct Soln1 {}
impl Soln1 {
    /// p0 = (0,0) v = (a,b)
    /// p1 = (a,b) v = (a-1,b-1)
    /// p2 = (a+a-1,b+b-1) v = (a-2,b-2)
    /// Ideally it should iteratively increase the number of steps
    /// while it still lands and stop simulating after
    /// Or even use a closed formish soln.
    pub fn part1(input: &str) -> isize {
        let ((xl, xh), (yl, yh)) = parse(input);
        let mut highest = isize::MIN;
        for (vx, vy) in iproduct!(-100..100, -100..100) {
            let traj = Self::trajectory(0, 0, vx, vy, 1000);
            let mut nsteps = 0;
            for &(px, py) in &traj {
                if px >= xl && px <= xh && py >= yl && py <= yh {
                    let traj_max = traj.iter().map(|p| p.1).max().unwrap();
                    if traj_max > highest {
                        highest = traj_max;
                    }
                }
                nsteps += 1
            }
        }
        highest
    }

    fn decr(n: isize) -> isize {
        match n {
            _n if n > 0 => _n - 1,
            _n if n < 0 => _n + 1,
            _ => 0,
        }
    }

    fn next(sx: isize, sy: isize, vx: isize, vy: isize) -> (isize, isize, isize, isize) {
        (sx + vx, sy + vy, Self::decr(vx), vy - 1)
    }

    pub fn trajectory(sx: isize, sy: isize, mut vx: isize, mut vy: isize, steps: usize) -> Vec<(isize, isize)> {
        let mut traj = Vec::new();
        let mut x = sx;
        let mut y = sy;
        for _ in 0..steps {
            traj.push((x, y));
            let (nx, ny, nvx, nvy) = Self::next(x, y, vx, vy);
            x = nx;
            y = ny;
            vx = nvx;
            vy = nvy;
        }
        traj
    }

    pub fn part2(input: &str) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::identity_op)]
    // use day17::{shared, soln1};
    // use day17::Soln1::trajectory;
    use super::Soln1;
    #[test]
    fn test_trajectory() {
        assert_eq!(
            Soln1::trajectory(0, 0, 7, 2, 10),
            vec![
                (0, 0),
                (7, 2),
                (7 + 6, 2 + 1),
                (7 + 6 + 5, 2 + 1 + 0),
                (7 + 6 + 5 + 4, 2 + 1 + 0 - 1),
                (7 + 6 + 5 + 4 + 3, 2 + 1 + 0 - 1 - 2),
                (7 + 6 + 5 + 4 + 3 + 2, 2 + 1 + 0 - 1 - 2 - 3),
                (7 + 6 + 5 + 4 + 3 + 2 + 1, 2 + 1 + 0 - 1 - 2 - 3 - 4),
                (7 + 6 + 5 + 4 + 3 + 2 + 1 + 0, 2 + 1 + 0 - 1 - 2 - 3 - 4 - 5),
                (7 + 6 + 5 + 4 + 3 + 2 + 1 + 0, 2 + 1 + 0 - 1 - 2 - 3 - 4 - 5 - 6),
            ]
        );

        println!("{:?}", Soln1::trajectory(0, 0, 6, 9, 10));
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = Soln1::part1(sample);
        println!("Part 1 (simulation) = {:?}", part1);
        assert_eq!(part1, 45);
    }

    #[test]
    fn test_part2() {
        // let sample: &str = include_str!("../inputs/sample.txt");
        // let part2 = soln1::Soln1::part2(sample);
        // println!("Part 2 (djikstra) = {:?}", part2);
        // assert_eq!(part2, 315);
    }
}
