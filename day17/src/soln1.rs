use crate::shared::parse;
use itertools::iproduct;

pub struct Soln1 {}
impl Soln1 {
    /// p0 = (0,0) v = (a,b)
    /// p1 = (a,b) v = (a-1,b-1)
    /// p2 = (a+a-1,b+b-1) v = (a-2,b-2)
    ///
    /// yn = b+(b-1)+(b-2)+...+(b-n+1)
    /// xn = a+(a-1)+(a-2)+...+max(0,(a-n+1))
    ///
    /// given a velocity (a,b), do binary search?
    /// as long as we can compute the max feasible timestep
    ///
    /// p(a,b,n) = (xn,yn)
    /// xn = a + (a-1) + ... + c
    ///
    /// p(a+1,b,n) =  (xn1,yn1)
    /// xn1 = a+1 + a + ... + c+1 = xn + 2
    ///
    /// vel 7
    /// 0 -> 7 -> 13
    /// vel 8
    /// 0 -> 8 -> 15
    ///
    /// max ht vel?
    /// vel reaches max ht at dy = 0
    /// if y = 7, dy = 0 after 7 steps
    /// max ht of v=7 = 7+6+5+4+3+2+1 = 7*8/2 = 28
    ///
    /// max ht of vy = vy*(vy-1)/2
    ///
    /// max ht of vy after n steps = vy...+(vy-n+1)
    /// (vx,vy) lands in (xl,xh),(yl,yh) when -
    ///
    /// S(a, i) = a + a-1 + ... + max(a-i+1,0)
    /// times when vx is in [xl,xh]
    /// i where xl <= S(vx, i) <= xh
    ///
    ///
    ///
    /// yn = vn - (1+2+3+..=(n-1))
    /// for v = 2
    ///     y0 = 0
    ///     y1 = 2*1 - 0 = 2
    ///     y2 = 2*2 - 1 = 3
    ///     y3 = 2*3 - 3 = 3
    /// so when is yn within a range [a..b]
    /// its the soln to a quadratic
    ///
    /// yn = vn - (n-1)*n/2
    ///
    /// yn = a
    ///
    /// vn - (n-1)*n/2 - a = 0
    /// 2vn - n^2 +n - a = 0
    /// -n^2 + n(2v+1) - a = 0
    /// n^2 - (2v+1)n + a = 0
    ///
    /// r1 = (-b + sqrt(b2-4ac))/2a = ((2v+1) + sqrt((2v+1)^2  - 4a))/2
    ///

    pub fn partial_sum(mut n: isize, mut i: usize, pos_only: bool) -> isize {
        let mut s = 0;
        while i > 0 {
            s += n;
            i -= 1;
            if pos_only && i == 0 {
                break;
            }
        }
        s
    }

    pub fn steps_in_range(v: isize, l: isize, h: isize) {
        if l > 0 && v < 0 || l < 0 && v > 0 {
            return None;
        }
    }

    pub fn part1(input: &str) -> isize {
        let ((xl, xh), (yl, yh)) = parse(input);
        let mut highest = isize::MIN;
        for (vx, vy) in iproduct!(-100..100, -100..100) {
            let traj = Self::trajectory(0, 0, vx, vy, 1000);
            for &(px, py) in &traj {
                if px >= xl && px <= xh && py >= yl && py <= yh {
                    let traj_max = traj.iter().map(|p| p.1).max().unwrap();
                    if traj_max > highest {
                        highest = traj_max;
                        println!("max ht {} reached by velocity {}", highest, vy);
                    }
                }
            }
        }
        highest
    }

    pub fn part2(input: &str) -> usize {
        let ((xl, xh), (yl, yh)) = parse(input);
        let mut nvalidtrajs = 0;
        for (vx, vy) in iproduct!(-1000..1000, -1000..1000) {
            let traj = Self::trajectory(0, 0, vx, vy, 1000);
            for &(px, py) in &traj {
                if px >= xl && px <= xh && py >= yl && py <= yh {
                    nvalidtrajs += 1;
                    break;
                }
            }
        }
        nvalidtrajs
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
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = Soln1::part2(sample);
        println!("Part 2 (simulation) = {:?}", part2);
        assert_eq!(part2, 112);
    }
}
