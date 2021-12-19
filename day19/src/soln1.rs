use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};

use crate::shared::{parse, Coord};

/// Naive implementation
/// Try every rotation
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

fn rotate(coord: Coord, around: Axis, turns: usize) -> Coord {
    let Coord(x, y, z) = coord;
    match around {
        Axis::X => match turns % 4 {
            0 => coord,
            1 => Coord(x, -z, y),
            n => rotate(coord, around, n - 1),
        },
        Axis::Y => match turns % 4 {
            0 => coord,
            1 => Coord(-z, y, x),
            n => rotate(coord, around, n - 1),
        },
        Axis::Z => match turns % 4 {
            0 => coord,
            1 => Coord(-y, x, z),
            n => rotate(coord, around, n - 1),
        },
    }
}

fn flip(coord: Coord, plane: (Axis, Axis)) -> Coord {
    let Coord(x, y, z) = coord;
    let (fp1, fp2) = (plane.0.min(plane.1), plane.0.max(plane.1));
    assert_ne!(fp1, fp2);
    match (fp1, fp2) {
        (Axis::X, Axis::Y) => Coord(x, y, -z),
        (Axis::X, Axis::Z) => Coord(x, -y, z),
        (Axis::Y, Axis::Z) => Coord(-x, y, z),
        _ => unreachable!(),
    }
}

fn flips(coord: Coord) -> Vec<Coord> {
    let planes = vec![(Axis::X, Axis::Y), (Axis::Y, Axis::Z), (Axis::X, Axis::Z)];
    let mut coords = vec![coord];
    for plane in planes {
        coords.push(flip(coord, plane))
    }
    coords
}

fn orientations(coord: Coord) -> Vec<Coord> {
    let mut orients = HashSet::new();
    for (xturns, yturns, zturns) in iproduct![0..4, 0..4, 0..4] {
        orients.insert(rotate(
            rotate(rotate(coord, Axis::X, xturns), Axis::Y, yturns),
            Axis::Z,
            zturns,
        ));
    }
    orients.into_iter().flat_map(flips).dedup().collect_vec()
}

fn translate(coord: Coord, by: &Coord) -> Coord {
    let Coord(x, y, z) = coord;
    let Coord(xo, yo, zo) = by;
    Coord(x + xo, y + yo, z + zo)
}

pub fn part1(input: &str) -> usize {
    let scan_coords = parse(input);
    let mut scanner = 1;
    let planes = vec![(Axis::X, Axis::Y), (Axis::Y, Axis::Z), (Axis::X, Axis::Z)];
    let mut plane_opts: Vec<Option<(Axis, Axis)>> = planes.iter().map(|p| Some((p.0, p.1))).collect_vec();
    plane_opts.push(None);

    let mut scan0coords: HashSet<&Coord> = HashSet::from_iter(scan_coords.get(&0).unwrap());

    let mut matched_pts: HashMap<usize, (usize, usize, usize, Coord, Option<(Axis, Axis)>, Vec<Coord>)> =
        HashMap::new();
    while scan_coords.contains_key(&scanner) {
        println!("computing matches with scanner {}", &scanner);
        let coords = scan_coords.get(&scanner).unwrap();
        'outer: for (xturns, yturns, zturns) in iproduct![0..4, 0..4, 0..4] {
            for plane_flip in &plane_opts {
                let mut matches = Vec::new();
                for translation_src in coords {
                    for translation_tgt in scan0coords.iter() {
                        for coord in coords {
                            let mut new_coord = rotate(
                                rotate(rotate(*coord, Axis::X, xturns), Axis::Y, yturns),
                                Axis::Z,
                                zturns,
                            );
                            if let Some(plane) = plane_flip {
                                new_coord = flip(new_coord, *plane)
                            }
                            //translate enough to make src land on tgt
                            //(sx+dx,sy+dy,sz+dz) = (tx,ty,tz)
                            //(dx,dy,dz) = (tx-sx,ty-sy,tz-sz)
                            let Coord(sx, sy, sz) = translation_src;
                            let Coord(tx, ty, tz) = translation_tgt;
                            let displace = Coord(tx - sx, ty - sy, tz - sz);
                            new_coord = translate(new_coord, &displace);

                            if scan0coords.contains(&new_coord) {
                                matches.push(*coord);
                            }
                            if matches.len() >= 12 {
                                let match_len = matches.len();
                                matched_pts.insert(scanner, (xturns, yturns, zturns, displace, *plane_flip, matches));
                                println!(
                                    "Scanner {} matched {} points using:\n
                                    displace({:?}
                                        flip({:?}
                                            rotate({},{},{})))",
                                    scanner, match_len, displace, plane_flip, xturns, yturns, zturns
                                );
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        scanner += 1;
    }
    //TODO:
    //- what if scanner 1 & 2 overlap some points that scanner 0 doesnt see?
    //- compute the normalized coords of beacons to generate a count
    0
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        shared::Coord,
        soln1::{part1, part2, Axis},
    };

    use super::orientations;

    #[test]
    fn test_orientations() {
        let coord = Coord(5, 6, -4);
        let mut orients = orientations(coord);
        orients.sort();

        println!("{} total orientations", orients.len());

        let mut expected = vec![
            Coord(5, 6, -4),
            Coord(-5, 4, -6),
            Coord(4, 6, 5),
            Coord(-4, -6, 5),
            Coord(-6, -4, -5),
        ];
        expected.sort();
        assert_eq!(orients, expected);
    }

    #[test]
    fn test_axis_order() {
        let axes = vec![Axis::X, Axis::Y, Axis::Z];
        let mut sorted_axes = axes.to_vec();
        sorted_axes.sort();
        assert_eq!(axes, sorted_axes);
    }

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = part1(sample);
        println!("Part 1 (sample1) = {:?}", part1);
        // assert_eq!(part1, 0);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = part2(sample);
        println!("Part 2 (sample2) = {:?}", part2);
        // assert_eq!(part2, 0);
    }
}
