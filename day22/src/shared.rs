use std::fmt::Debug;

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Interval(pub isize, pub isize);
impl Interval {
    fn intersect(&self, other: &Self) -> Option<Interval> {
        let Interval(l1, h1) = self;
        let Interval(l2, h2) = other;

        let lc = *l1.max(l2);
        let rc = *h1.min(h2);
        if (lc <= rc) {
            Some(Interval(lc, rc))
        } else {
            None
        }
        //(-4,-3), (-10,-6)
        // if h1 > l2 && l1 <= {
        // Some(Interval(*l2, *h1))
        // } else {
        // None
        // }
    }

    #[inline]
    pub fn inner_len(&self) -> usize {
        ((self.1 - self.0) as usize) + 1
    }

    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}
impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}
impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0).then(self.1.cmp(&other.1)))
    }
}
impl Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", &self.0, &self.1)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Box {
    pub x_range: Interval,
    pub y_range: Interval,
    pub z_range: Interval,
}
impl Box {
    #[inline]
    pub fn volume(&self) -> usize {
        self.x_range.inner_len() * self.y_range.inner_len() * self.z_range.inner_len()
    }

    #[inline]
    pub fn contains(&self, other: &Box) -> bool {
        self.x_range.contains(&other.x_range)
            && self.y_range.contains(&other.y_range)
            && self.z_range.contains(&other.z_range)
    }

    pub fn intersect(&self, other: &Box) -> Option<Box> {
        if let Some(xint) = &self.x_range.intersect(&other.x_range) {
            if let Some(yint) = &self.y_range.intersect(&other.y_range) {
                if let Some(zint) = &self.z_range.intersect(&other.z_range) {
                    return Some(Box {
                        x_range: *xint,
                        y_range: *yint,
                        z_range: *zint,
                    });
                }
            }
        }
        None
    }
}
impl Debug for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box[x={:?},y={:?},z={:?}",
            &self.x_range, &self.y_range, &self.z_range
        )
    }
}

pub fn parse(input: &str) -> Vec<(i8, Box)> {
    //on x=-20..26,y=-36..17,z=-47..7

    let mut cubes = Vec::new();
    let lines = input.lines();
    for line in lines {
        let (p, q) = line.split_once(' ').unwrap();
        let pb = {
            if p == "on" {
                1
            } else {
                -1
            }
        };
        let r = q.split(',').collect_vec();
        let spl = (r[0].split("..").collect_vec());
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();
        let xl = splh.parse::<isize>().unwrap();
        let xh = spl[1].parse::<isize>().unwrap();
        let spl = r[1].split("..").collect_vec();
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();
        let yl = splh.parse::<isize>().unwrap();
        let yh = spl[1].parse::<isize>().unwrap();
        let spl = r[2].split("..").collect_vec();
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();

        let zl = splh.parse::<isize>().unwrap();
        let zh = spl[1].parse::<isize>().unwrap();

        let row = (
            pb,
            Box {
                x_range: Interval(xl.min(xh), xl.max(xh)),
                y_range: Interval(yl.min(yh), yl.max(yh)),
                z_range: Interval(zl.min(zh), zl.max(zh)),
            },
        );
        cubes.push(row);
    }
    cubes
}

#[cfg(test)]
mod tests {
    use crate::shared::Box;
    use crate::shared::Interval as I;

    use super::parse;

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        assert_eq!(parsed.len(), 22);
        assert_eq!(
            parsed[1],
            (
                1,
                Box {
                    x_range: I(-20, 33),
                    y_range: I(-21, 23),
                    z_range: I(-26, 28)
                }
            )
        );
    }

    #[test]
    fn test_intersect_interval() {
        let a = I(-4, -3);
        let b = I(-1, 3);
        let c = I(-2, 5);
        let d = I(4, 10);
        let e = I(-10, -6);

        assert_eq!(a.intersect(&b), None);
        assert_eq!(a.intersect(&c), None);
        assert_eq!(a.intersect(&d), None);
        assert_eq!(a.intersect(&e), None);

        assert_eq!(b.intersect(&c), Some(b));
        assert_eq!(b.intersect(&d), None);
        assert_eq!(b.intersect(&e), None);

        assert_eq!(c.intersect(&d), Some(I(4, 5)));
        assert_eq!(c.intersect(&e), None);

        assert_eq!(d.intersect(&c), Some(I(4, 5)));
        assert_eq!(d.intersect(&e), None);

        // let a = I(3,3);
        // let b = I(5,5);
    }

    #[test]
    fn test_intersect_box() {
        let a = Box {
            x_range: I(0, 5),
            y_range: I(0, 5),
            z_range: I(0, 5),
        };
        let b = Box {
            x_range: I(0, 3),
            y_range: I(0, 3),
            z_range: I(0, 3),
        };
        assert_eq!(a.intersect(&b), Some(b.clone()));
        assert_eq!(b.intersect(&a), Some(b));

        let c = Box {
            x_range: I(-1, 1),
            y_range: I(-1, 1),
            z_range: I(-1, 1),
        };
        let d = Box {
            x_range: I(0, 1),
            y_range: I(0, 1),
            z_range: I(0, 1),
        };
        assert_eq!(c.intersect(&d), Some(d.clone()));
        assert_eq!(d.intersect(&c), Some(d.clone()));
    }
}
