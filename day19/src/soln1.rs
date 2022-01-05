use crate::shared::{parse, Coord};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Rotate(Axis, usize),
    Ident,
}
impl Op {
    fn rotate(coord: &Coord, around: Axis, turns: usize) -> Coord {
        let Coord(x, y, z) = *coord;
        match around {
            Axis::X => match turns % 4 {
                0 => *coord,
                1 => Coord(x, -z, y),
                n => Self::rotate(&Self::rotate(coord, around, n - 1), around, 1),
            },
            Axis::Y => match turns % 4 {
                0 => *coord,
                1 => Coord(-z, y, x),
                n => Self::rotate(&Self::rotate(coord, around, n - 1), around, 1),
            },
            Axis::Z => match turns % 4 {
                0 => *coord,
                1 => Coord(-y, x, z),
                n => Self::rotate(&Self::rotate(coord, around, n - 1), around, 1),
            },
        }
    }

    pub fn apply(&self, point: &Coord) -> Coord {
        match self {
            Op::Rotate(axis, nturns) => Self::rotate(point, *axis, *nturns),
            Op::Ident => *point,
        }
    }

    pub fn inverse(&self, point: &Coord) -> Coord {
        match self {
            Op::Rotate(axis, nturns) => (Op::Rotate(*axis, 4 - nturns)).apply(point),
            Op::Ident => *point,
        }
    }
}

#[derive(Debug, Clone)]
struct Ops {
    ops: Vec<Op>,
}
impl Ops {
    fn apply(&self, point: Coord) -> Coord {
        let mut new_point = point;
        for op in &self.ops {
            new_point = op.apply(&new_point)
        }
        new_point
    }

    pub fn inverse(&self, point: Coord) -> Coord {
        let mut new_point = point;
        for op in self.ops.iter().rev() {
            new_point = op.inverse(&new_point)
        }
        new_point
    }
}

#[derive(Debug, Clone)]
// Position in space + rotation along 3 axes
struct ScannerPos(Coord, Ops);

fn get_overlaps(seer_points: &Vec<Coord>, ref_points: &Vec<Coord>, ref_origin: &Coord) -> (usize, Coord) {
    let ref_points_absolute = ref_points.iter().map(|c| c.add(ref_origin)).collect_vec();
    for point in seer_points {
        for ref_point in &ref_points_absolute {
            //point + diff = (p.x + d.x, ...) = (p.x + r.x - p.x, ...) = (r.x, ...)
            let diff = ref_point.add(&point.neg());
            let candidate_origin = diff;

            let mapped_seers = seer_points.iter().map(|c| c.add(&candidate_origin)).collect_vec();
            let mut intersect_count = 0;
            for ref_point_again in &ref_points_absolute {
                if mapped_seers.iter().contains(ref_point_again) {
                    intersect_count += 1;
                }
            }
            if intersect_count >= 12 {
                return (intersect_count, candidate_origin);
            }
        }
    }
    (0, Coord(0, 0, 0))
}

pub fn part1(input: &str) -> HashSet<Coord> {
    let scan_coords = parse(input);
    let located_scanners = locate_scanners(&scan_coords);
    println!("Located all scanners!");

    let full_normalize = |coord: &Coord, pos: &ScannerPos| -> Coord {
        let scanner_origin = pos.0;
        let scanner_rotations = &pos.1;
        let unrotate = scanner_rotations.inverse(*coord);
        let translate = unrotate.add(&scanner_origin);
        translate
    };

    let mut distinct_coords = HashSet::new();
    for (scanner_idx, scanner_pts) in scan_coords {
        let normalized_coords =
            scanner_pts.iter().map(|c| full_normalize(c, located_scanners.get(&scanner_idx).unwrap()));
        for coord in normalized_coords {
            distinct_coords.insert(coord);
        }
    }
    println!("Found {} distinct coords!", distinct_coords.len());
    distinct_coords
}

fn locate_scanners(scan_coords: &HashMap<usize, Vec<Coord>>) -> HashMap<usize, ScannerPos> {
    let num_scanners = scan_coords.len();

    let mut located_scanners: HashMap<usize, ScannerPos> =
        HashMap::from_iter([(0, ScannerPos(Coord(0, 0, 0), Ops { ops: [Op::Ident; 3].to_vec() }))]);

    let mut x_rotates = (0..4).map(|nt| Op::Rotate(Axis::X, nt)).collect_vec();
    let mut y_rotates = (0..4).map(|nt| Op::Rotate(Axis::Y, nt)).collect_vec();
    let mut z_rotates = (0..4).map(|nt| Op::Rotate(Axis::Z, nt)).collect_vec();
    let axes = vec![Axis::X, Axis::Y, Axis::Z];

    let normalize_coords = |scanner_idx: usize, ops: &Ops| -> Vec<Coord> {
        let seen = scan_coords.get(&scanner_idx).unwrap();
        seen.iter().map(|coord| ops.inverse(*coord)).collect_vec()
    };

    let mut num_located = located_scanners.len();
    while num_located != num_scanners {
        let unlocated = (0..num_scanners).filter(|idx| !located_scanners.contains_key(idx));
        'unlocated: for unlocated_scanner in unlocated {
            for (rotate_x, rotate_y, rotate_z) in iproduct![&x_rotates, &y_rotates, &z_rotates] {
                let rotates = Ops { ops: vec![*rotate_x, *rotate_y, *rotate_z] };
                let normalized_seen = normalize_coords(unlocated_scanner, &rotates);
                for (sc_idx, sc_pos) in &located_scanners {
                    let normalized_ref = normalize_coords(*sc_idx, &sc_pos.1);
                    let (num_overlaps, seer_pos) = get_overlaps(&normalized_seen, &normalized_ref, &sc_pos.0);
                    if num_overlaps >= 12 {
                        let scanner_pos = ScannerPos(seer_pos, rotates.clone());
                        eprintln!(
                            "Located scanner #{} at {:?} . {} overlaps with scanner #{}",
                            unlocated_scanner, &scanner_pos, num_overlaps, sc_idx
                        );
                        num_located += 1;
                        located_scanners.insert(unlocated_scanner, scanner_pos);
                        break 'unlocated;
                    }
                }
            }
        }
    }
    located_scanners
}

pub fn part2(input: &str) -> usize {
    let scan_coords = parse(input);
    let located_scanners = locate_scanners(&scan_coords);

    let mut max_dist = 0;
    for ScannerPos(origin1, _) in located_scanners.values() {
        for ScannerPos(origin2, _) in located_scanners.values() {
            max_dist = max_dist.max(origin1.l1_dist(origin2));
        }
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use Axis::*;
    use Op::*;

    #[test]
    fn test_op_apply() {
        let test_points = vec![Coord(1, 2, 3)];
        let twice_x = Ops { ops: vec![Rotate(X, 1), Rotate(X, 1)] };
        let x2 = Ops { ops: vec![Rotate(X, 2)] };
        for point in &test_points {
            assert_eq!(twice_x.apply(*point), x2.apply(*point))
        }
    }

    #[test]
    fn test_op_inverse() {
        let test_points = vec![Coord(1, 2, 3)];
        let twice_x = Ops { ops: vec![Rotate(X, 1), Rotate(X, 1)] };
        let test_ops = vec![twice_x];

        for point in test_points.iter() {
            for op in test_ops.iter() {
                let roundtrip = op.inverse(op.apply(*point));
                assert_eq!(*point, roundtrip);
            }
        }
    }

    #[test]
    fn test_axis_order() {
        let axes = vec![Axis::X, Axis::Y, Axis::Z];
        let mut sorted_axes = axes.to_vec();
        sorted_axes.sort();
        assert_eq!(axes, sorted_axes);
    }

    #[test]
    fn test_scanner_locator() {
        let sample = include_str!("../inputs/sample.txt");
        let scan_coords = parse(sample);
        let locations = locate_scanners(&scan_coords);
        assert_eq!(locations.get(&0).unwrap().0, Coord(0, 0, 0));
        assert_eq!(locations.get(&1).unwrap().0, Coord(68, -1246, -43));
        assert_eq!(locations.get(&2).unwrap().0, Coord(1105, -1205, 1229));
        assert_eq!(locations.get(&3).unwrap().0, Coord(-92, -2380, -20));
        assert_eq!(locations.get(&4).unwrap().0, Coord(-20, -1133, 1061));
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = part1(sample);
        assert_eq!(part1.len(), 79);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = part2(sample);
        assert_eq!(part2, 3621);
    }
}
