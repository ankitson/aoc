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

    fn count_set(cubes: [[[i8; 101]; 101]; 101]) -> usize {
        let mut nset = 0;
        for x in 0..=100 {
            for y in 0..=100 {
                for z in 0..=100 {
                    nset += if cubes[x][y][z] == 1 {
                        cubes[x][y][z] as usize
                    } else {
                        0
                    }
                }
            }
        }
        nset
    }

    pub fn part1(input: &str) -> usize {
        let cubes = parse(input);
        let mut map = [[[0i8; 101]; 101]; 101];
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
     **/

    /**
     * |A u B| = |A| + |B| - |A v B|
     *
     *
     */
    pub fn part2(input: &str) -> usize {
        let mut boxes = parse(input);
        let mut current: Vec<(i8, Box)> = vec![];

        let mut pcount = 0;
        for (state, cube) in &boxes {
            let mut to_add = if *state == 1 { vec![(1, cube.clone())] } else { vec![] };
            for (ostate, ocube) in &current {
                if let Some(intersect) = (ocube).intersect(cube) {
                    if pcount < 50 {
                        println!("{},{:?} intsct {},{:?} = {:?}", state, cube, ostate, ocube, intersect);
                        pcount += 1;
                    }
                    let fstate = if (*state == 1 && *ostate == 1) {
                        -1
                    } else if *state == -1 && *ostate == -1 {
                        1
                    } else {
                        *state
                    };
                    to_add.push((fstate, intersect))
                }
            }
            current.extend(to_add)
        }

        println!("Final cubes:");
        for (state, cube) in &current {
            println!(
                "{} {:?} volume = {}",
                *state,
                *cube,
                *state as isize * cube.volume() as isize
            );
        }

        let mut on_volume: isize = 0;
        for (state, cube) in current {
            // println!("on vol: {} incoming: {} {:?}", on_volume, state, cube);
            on_volume += (state as isize) * (isize::try_from(cube.volume()).unwrap())
        }
        on_volume as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/day22.txt");
        assert_eq!(Soln1::part1(sample), 588200);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample3.txt");
        assert_eq!(Soln1::part2(sample), 2758514936282235);
    }

    #[test]
    fn test_simple() {
        let sample2 = include_str!("../inputs/sample2.txt");
        assert_eq!(Soln1::part2(sample2), 5 * 5 * 5);
    }
}
