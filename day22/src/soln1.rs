use crate::shared::parse;
use itertools::Itertools;
use std::iter::repeat;

type Cubes = Vec<(u8, isize, isize, isize, isize, isize, isize)>;
#[derive(Clone, Debug)]
pub struct Node {
    xmid: isize,
    ymid: isize,
    zmid: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>, // children: Vec<(Option<Box<Node>>, usize)>, // left: &'a mut Option<Box<Node<'a>>>,                                         // right: &'a Option<Box<Node<'a>>>
}
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

    fn surface_str(cube: (isize, isize, isize, isize, isize, isize), num: usize) -> String {
        let (xl, xh, yl, yh, zl, zh) = cube;
        match num {
            0 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xl, yl, zl, xl, yh, zl, xl, yh, zh, xl, yl, zh, xl, yl, zl
            ), //X=Low, XZ plane
            1 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xh, yh, zh, xh, yh, zl, xh, yl, zl, xh, yl, zh, xh, yh, zh
            ), //X=Hi, XZ plane
            2 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xl, yl, zl, xh, yl, zl, xh, yl, zh, xl, yl, zh, xl, yl, zl
            ), //Y=Low, XZ plane
            3 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xh, yh, zh, xl, yh, zh, xl, yh, zl, xh, yh, zl, xh, yh, zh
            ), //Y=Hi, XZ plane
            4 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xl, yl, zl, xh, yl, zl, xh, yh, zl, xl, yh, zl, xl, yl, zl
            ), //Z=Low, XY plane
            5 => format!(
                "({} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {})",
                xh, yh, zh, xl, yh, zh, xl, yl, zh, xh, yl, zh, xh, yh, zh
            ), //Z=Hi, XY plane
            _ => panic!(),
        }
    }

    fn polyhedra_str(cube: (isize, isize, isize, isize, isize, isize)) -> String {
        let mut str: String = String::new();
        str.push_str("POLYHEDRALSURFACE Z (\n");
        for i in vec![0, 2, 4, 1, 3, 5] {
            let lin = Self::surface_str(cube, i);
            str.push_str(&format!("({})", &lin).to_string());
            if i != 5 {
                str.push_str(",")
            }
            str.push_str("\n");
        }
        str.push_str(")");
        str
    }

    pub fn part2(input: &str) -> usize {
        let max = 100000;
        let cubes = parse(input);
        for cube in cubes {
            println!("cube: {:?}", cube);
            println!(
                "{}",
                Self::polyhedra_str((cube.1, cube.2, cube.3, cube.4, cube.5, cube.6))
            );
        }
        todo!()
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
