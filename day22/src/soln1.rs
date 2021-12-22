use crate::shared::parse;
use itertools::Itertools;
use std::iter::repeat;

type Cubes = Vec<(u8, isize, isize, isize, isize, isize, isize)>;
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
        for (stat, xl, xh, yl, yh, zl, zh) in cubes {
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

    pub fn part2(input: &str) -> usize {
        todo!()
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
