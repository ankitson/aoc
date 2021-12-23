use crate::shared::parse;
use crate::shared::Box;
use crate::shared::Interval as I;
use itertools::Itertools;
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

    // fn count_set2(cubes: [[[u8; 2 * 100_000 + 1]; 2 * 100_000 + 1]; 2 * 100_000 + 1]) -> usize {
    //     let mut nset = 0;
    //     let max = 2 * 100_000 + 1;
    //     for x in 0..=max {
    //         for y in 0..=max {
    //             for z in 0..=max {
    //                 nset += (cubes[x][y][z] as usize);
    //             }
    //         }
    //     }
    //     nset
    // }

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

    //     -- I. b and a are positive
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
    // -- III. a and b are negative
    //  ---- volume = 0

    pub fn part2(input: &str) -> usize {
        let max = 100000;
        let mut cubes = parse(input);

        impl PartialOrd for Box {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Box {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.x_range
                    .cmp(&other.x_range)
                    .then(self.y_range.cmp(&other.y_range))
                    .then(self.z_range.cmp(&other.z_range))
            }
        }

        //WONT WORK without changes because this does not account for 
        //the order in which cubes appear
        cubes.sort_by(|t1, t2| t1.1.cmp(&t2.1));
        let mut prevt = None;
        let mut volume = 0;
        for (status, cube) in cubes {
            if prevt.is_none() {
                volume += if status == 0 { 0 } else { cube.volume() };
                prevt = Some((status, cube));
                continue;
            }
            let (pstatus, prev) = prevt.unwrap();
            if prev.contains(&cube) {
                if (pstatus == 1 && status == 1) {
                    //
                } else if (pstatus == 0 && status == 1) {
                    volume += cube.volume()
                } else if (pstatus == 1 && status == 0) {
                    volume -= cube.volume()
                } else {
                    //
                }
            } else if cube.contains(&prev) {
                if (pstatus == 1 && status == 1) {
                    volume += cube.volume() - prev.volume()
                } else if (pstatus == 0 && status == 1) {
                    volume += cube.volume()
                } else if (pstatus == 1 && status == 0) {
                    volume -= cube.volume()
                } else {

            }

            // println!(
            //     "{}",
            //     Self::polyhedra_str((cube.1, cube.2, cube.3, cube.4, cube.5, cube.6))
            // );
            prevt = Some((status, cube));
        }
        0
        // Self::count_set2(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::parse;

    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
    }
}
