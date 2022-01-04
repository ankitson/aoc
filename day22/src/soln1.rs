use crate::shared::parse;
use crate::shared::Box;
use crate::shared::Interval as I;
use itertools::Itertools;
use sorted_vec::SortedVec;
use std::iter::repeat;

pub struct Soln1 {}
impl Soln1 {
    fn out_bounds(cc: isize) -> bool {
        cc < -50 || cc > 50
    }

    fn count_set(cubes: [[[u8; 101]; 101]; 101]) -> usize {
        let mut nset = 0;
        for x in 0..=100 {
            for y in 0..=100 {
                for z in 0..=100 {
                    nset += (cubes[x][y][z] as usize);
                }
            }
        }
        nset
    }

    pub fn part1(input: &str) -> usize {
        let cubes = parse(input);
        let mut map = [[[0u8; 101]; 101]; 101];
        for (
            stat,
            Box {
                x_range: I(xl, xh),
                y_range: I(yl, yh),
                z_range: I(zl, zh),
            },
        ) in cubes
        {
            if (Self::out_bounds(xl)
                || Self::out_bounds(xh)
                || Self::out_bounds(yl)
                || Self::out_bounds(yh)
                || Self::out_bounds(zl)
                || Self::out_bounds(zh))
            {
                continue;
            }
            for x in xl..=xh {
                for y in yl..=yh {
                    for z in zl..=zh {
                        let xi: usize = (x + 50) as usize;
                        let yi: usize = (y + 50) as usize;
                        let zi: usize = (z + 50) as usize;
                        map[xi][yi][zi] = stat;
                    }
                }
            }
        }
        Self::count_set(map)
    }

    // --  I. b and a are positive
    // ---- 1. b is contained in a
    //  ---- test: st_3dintersection(a,b) == b
    //  ---- volume =  volume(a)
    // ---- 2. a is contained in b
    //  ---- volume = volume(b)
    // ---- 3. a/b are disjoint
    //  ---- volume = volume(a) + volume(b)
    // ---- 3. a/b intersect in a cuboid
    //  ---- volume = volume(a) + volume(b) - volume(intersect(a,b))
    // -- II. b is negative, a is positive
    // ---- 1. b is contained in a
    //  ---- volume = volume(a) - volume(b)
    //  ---- 1. a is contained in b
    //  ---- volume = 0
    // ---- 2. disjoint from a
    //  ---- volume = volume(a)
    // ---- 3. intersects
    //  ---- volume = volume(a) - volume(intersect(a,b))
    // --III. a and b are negative
    //  ---- volume = 0

    /**
     * Ideal 1D interval
     *
     * store 1 point at each transition
     *
     *      ON         OFF     ON       OFF    ON
     * 0-----------10------12-------15------20-----24
     *
     * 2D version
     *
     * ---------XXXXXXX-------XXXXX
     * ---------XXXXXXX-------XXXXX
     * ------XXXXXXX--------XXXX---
     * ------XXXXXXX--------XXXX---
     * ------XXXXX----------XX-----
     *
     * ------T--T-T-T--T----T-T-T--   Xes
     */

    pub fn part2(input: &str) -> usize {
        let mut boxes = parse(input);
        let mut current = vec![];

        let mut on_volume = 0;
        for (state, cube) in boxes {
            on_volume += if state == 1 { cube.volume() } else { 0 };
            for (ostate, ocube) in &current {
                match (ostate, state) {
                    (0, 0) => (),
                    (0, 1) => (),
                    (1, 0) => on_volume -= cube.intersect(ocube).map(|b| b.volume()).unwrap_or(0),
                    (1, 1) => on_volume -= cube.intersect(ocube).map(|b| b.volume()).unwrap_or(0),
                    _ => unreachable!(),
                }
            }
            current.push((state, cube));
        }

        on_volume
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        assert_eq!(Soln1::part1(sample), 590784);
    }
}
