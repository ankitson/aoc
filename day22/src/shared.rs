use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Interval(pub isize, pub isize);
impl Interval {
    fn intersect(&self, other: &Self) -> Option<Interval> {
        let Interval(l1, h1) = self;
        let Interval(l2, h2) = other;
        if h1 > l2 {
            Some(Interval(*l2, *h1))
        } else {
            None
        }
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
#[derive(PartialEq, Eq, Debug, Clone)]
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
pub fn parse(input: &str) -> Vec<(u8, Box)> {
    //on x=-20..26,y=-36..17,z=-47..7

    let mut cubes = Vec::new();
    let lines = input.lines();
    for line in lines {
        let (p, q) = line.split_once(' ').unwrap();
        let pb = {
            if p == "on" {
                1
            } else {
                0
            }
        };
        let r = q.split(',').collect_vec();
        let spl = (r[0].split("..").collect_vec());
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();
        println!("splh: {:?}", spl);
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
                x_range: Interval(xl, xh),
                y_range: Interval(yl, yh),
                z_range: Interval(zl, zh),
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
}
