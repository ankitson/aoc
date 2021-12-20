use crate::shared::{parse, Coord};
use itertools::{iproduct, Itertools, izip};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]

struct Plane(Axis, Axis);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Rotate(Axis, usize),
    Flip(Plane),
    Translate(Coord),
    Ident
}
impl Op {
    fn rotate(coord: &Coord, around: Axis, turns: usize) -> Coord {
        let Coord(x, y, z) = *coord;
        match around {
            Axis::X => match turns % 4 {
                0 => *coord,
                1 => Coord(x, -z, y),
                n => Self::rotate(&Self::rotate(coord, around, n - 1),around, 1)
            },
            Axis::Y => match turns % 4 {
                0 => *coord,
                1 => Coord(-z, y, x),
                n => Self::rotate(&Self::rotate(coord, around, n - 1),around, 1)
            },
            Axis::Z => match turns % 4 {
                0 => *coord,
                1 => Coord(-y, x, z),
                n => Self::rotate(&Self::rotate(coord, around, n - 1),around, 1)
            },
        }
    }

    fn flip(coord: &Coord, plane: &Plane) -> Coord {
        let Coord(x, y, z) = *coord;
        let (fp1, fp2) = (plane.0.min(plane.1), plane.0.max(plane.1));
        assert_ne!(fp1, fp2);
        match (fp1, fp2) {
            (Axis::X, Axis::Y) => Coord(x, y, -z),
            (Axis::X, Axis::Z) => Coord(x, -y, z),
            (Axis::Y, Axis::Z) => Coord(-x, y, z),
            _ => panic!("bad plane"),
        }
    }

    pub fn apply(&self, point: &Coord) -> Coord {
        match self {
            Op::Rotate(axis, nturns) => Self::rotate(point, *axis, *nturns),
            Op::Flip(plane) => Self::flip(point, plane),
            Op::Translate(coord) => point.translate(coord),
            Op::Ident => *point
        }
    }

    pub fn inverse(&self, point: &Coord) -> Coord {
        match self {
            Op::Rotate(axis, nturns) => (Op::Rotate(*axis, 4 - nturns)).apply(point),
            Op::Flip(plane) => Op::Flip(*plane).apply(point),
            Op::Translate(coord) => point.translate(&Coord(-coord.0,-coord.1,-coord.2)),
            Op::Ident => *point
        }
    }
}

#[derive(Debug)]
struct Ops {
    ops: Vec<Op>,
}
impl Ops {
    fn apply(&self, point: Coord) -> Coord {
        let mut new_point = point.clone();
        for op in &self.ops {
            new_point = op.apply(&new_point)
        }
        new_point
    }

    pub fn inverse(&self, point: Coord) -> Coord {
        let mut new_point = point.clone();
        for op in self.ops.iter().rev() {
            new_point = op.inverse(&new_point)
        }
        new_point
    }
}


pub fn part1(input: &str) -> usize {
    let scan_coords = parse(input);
    let num_scanners = scan_coords.len();

    let axes = vec![Axis::X, Axis::Y, Axis::Z];
    let planes = vec![Plane(Axis::X,Axis::Y),Plane(Axis::X,Axis::Z),Plane(Axis::Y,Axis::Z)];
    let mut x_rotates = (0..4).map(|nt| Op::Rotate(Axis::X, nt)).collect_vec();
    let mut y_rotates = (0..4).map(|nt| Op::Rotate(Axis::Y, nt)).collect_vec();
    let mut z_rotates = (0..4).map(|nt| Op::Rotate(Axis::Z, nt)).collect_vec();
    let mut flips = planes.iter().map(|p| Op::Flip(*p)).collect_vec();
        flips.push(Op::Ident);
    let flips = flips;

    let mut z_flip = vec![Op::Flip(Plane(Axis::X,Axis::Y)), Op::Ident];
    let mut y_flip = vec![Op::Flip(Plane(Axis::X,Axis::Z)), Op::Ident];
    let mut x_flip = vec![Op::Flip(Plane(Axis::Y,Axis::Z)), Op::Ident];


    let mut scposes: HashMap<(usize,usize),(Ops, Coord)> = HashMap::new();

    for (sc1, sc2) in iproduct!(0..num_scanners, 0..num_scanners) {
        if sc1 >= sc2 {
            continue;
        }
        let sc1_points = scan_coords.get(&sc1).unwrap();
        let sc2_points = scan_coords.get(&sc2).unwrap();
        for (rotate_x,rotate_y,rotate_z) in izip![&x_rotates,&y_rotates,&z_rotates] {
            for (flip1,flip2,flip3) in iproduct![&x_flip,&y_flip,&z_flip] {
                for (p1, p2) in iproduct![sc1_points, sc2_points] {
                    let op_chain = Ops { ops: vec![*rotate_x, *rotate_y, *rotate_z, *flip1, *flip2, *flip3] };
                    let p2_inv = op_chain.inverse(*p2);
                    // let translation = Op::Translate(Coord(-p2_inv.0,-p2_inv.1,-p2_inv.2));

                    let candidate_origin2 = Coord(p1.0-p2_inv.0,p1.1-p2_inv.1,p1.2-p2_inv.2);
                    // if candidate_origin2.l0_dist() > 1000 {
                        // continue;
                    // }
                    // if sc1 == 0 && sc2 == 1 && *p1 == Coord(-618,-824,-621) {
                        // println!("cand origin {:?}", candidate_origin2);
                    // }
                    // if candidate_origin2 == Coord(68,-1246,-43) {//*p1 == Coord(-618,-824,-621) && *p2 == Coord(686,422,578)) {
                    //     // println!("calibrating {}/{} using points {:?}/{:?}={:?}", sc1, sc2, p1,p2,p2_inv);
                    //     let overlaps = compute_overlaps(sc1_points, sc2_points, &op_chain, candidate_origin2);
                    //     if overlaps > 0 {
                    //         // println!("calibrating {}/{} using points {:?}/{:?}={:?}", sc1, sc2, p1,p2,p2_inv);
                    //         // println!("op chain: {:?} origin2: {:?}", op_chain, candidate_origin2);
                    //         // println!("got {} overlaps", overlaps);
                    //     }
                    // }
                    let overlaps = compute_overlaps(sc1_points, sc2_points, &op_chain, candidate_origin2);
                    // if (sc1 == 1 && sc2 == 4) {
                    //     println!("1 & 4 have {} overlaps", overlaps)
                    // }
                    if overlaps >= 12 && !scposes.contains_key(&(sc1,sc2)) {
                        let lower_ref = scposes.keys().find(|(p,c)| *p < sc1 && *c == sc2);
                        if lower_ref.is_none() {
                            scposes.insert((sc1,sc2), (op_chain, candidate_origin2));
                        }
                    }
                    // if sc1 == 0 && sc2 == 1 {
                    //     println!("scanner {}/{} = {} overlaps assuming orient {:?}, pos {:?}", sc1, sc2, overlaps, op_chain, candidate_origin2);
                    // }
                }
            }
        }
    }

    println!("scposes keys: {:?}", scposes.keys());
    
    // let mut abs_poses = HashMap::new();
    let mut parent_chains = HashMap::new();
    for (k,v) in scposes.iter() {
        let (sc1,sc2) = k;
        if *sc2 == 0 {
            continue;
        }
        let mut parent_chain = vec![sc1];
        while **parent_chain.last().unwrap() != 0 {
            let parent = **parent_chain.last().unwrap();
            let grand_parent = scposes.keys().filter(|(p,c)| *c == parent).map(|(p,c)| p).min().unwrap();
            parent_chain.push(grand_parent);
        }
        parent_chains.insert(sc2, parent_chain);
    }
    for i in (1..num_scanners).rev() {
        if let Some(parent_chain) = parent_chains.get(&i) {
            let mut current_parent = parent_chains.get(&i).unwrap();
            
        }
    }
    println!("built parent chains: {:?}", parent_chains);


    0
}

fn compute_overlaps(pts1: &Vec<Coord>, pts2: &Vec<Coord>, ops: &Ops, origin2: Coord) -> usize {
    let pts1_set: HashSet<Coord> = HashSet::from_iter(pts1.iter().cloned());
    let mut noverlaps = 0;

    // let p21 = Coord(0,0,0);
    // let p21_inv = ops.inverse(p21);
    // let p21_wrt_0 = Coord(p21_inv.0 + origin2.0, p21_inv.1 + origin2.1, p21_inv.2 + origin2.2);
    // println!("P2 origin w.r.t P1 with {:?} is at {:?}", ops, p21_wrt_0);
    for p2 in pts2 {
        let p2_inv = ops.inverse(*p2);
        let p2_wrt_p1 = Coord(p2_inv.0+origin2.0, p2_inv.1+origin2.1, p2_inv.2+origin2.2);
        // if (origin2 == Coord(68,-1246,-43)) {
            // println!("candidate origin2 hitter at ops: {:?}", ops);
            // println!("p2 = {:?} inv(p2) = {:?} translate(inv(p2)) = {:?}", p2, p2_inv, p2_wrt_p1)    
        // }
        if pts1_set.contains(&p2_wrt_p1) {
            noverlaps += 1;
            // if noverlaps > 1 && *p2 == Coord(686,422,575) {
                // println!("p2 wrt p1 = {:?}", p2_wrt_p1);
            // }
        }
    }
    noverlaps
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        shared::Coord,
        soln1::{part1, part2},
    };

    use super::{Axis, Axis::X, Axis::Y, Axis::Z, Op, Op::Flip, Op::Rotate, Ops, Plane};

    #[test]
    fn test_op_apply() {
        //1,2,3 -> 1,-3,2 -> 1,-2,-3
        let test_points = vec![Coord(1, 2, 3)];
        let twice_x = Ops {
            ops: vec![Rotate(X, 1), Rotate(X, 1)],
        };
        let x2 = Ops {
            ops: vec![Rotate(X, 2)],
        };
        for point in &test_points {
            assert_eq!(twice_x.apply(*point), x2.apply(*point))
        }

        let two_flip = Ops {
            ops: vec![Flip(Plane(X, Y)), Flip(Plane(X, Y))],
        };
        for point in test_points.iter() {
            assert_eq!(two_flip.apply(*point), *point)
        }

        //[1,2,3] -> [1,2,-3] -> [-2,1,-3]
        let flip_rotate = Ops {
            ops: vec![Flip(Plane(X,Y)),Rotate(Z,1)]
        };
        assert_eq!(flip_rotate.apply(test_points[0]), Coord(-2,1,-3))
    }

    #[test]
    fn test_op_inverse() {
        let test_points = vec![Coord(1, 2, 3)];
        let twice_x = Ops {
            ops: vec![Rotate(X, 1), Rotate(X, 1)],
        };
        let two_flip = Ops {
            ops: vec![Flip(Plane(X, Y)), Flip(Plane(X, Y))],
        };
        //[1,2,3] -> [1,2,-3] -> [-2,1,-3]
        let flip_rotate = Ops {
            ops: vec![Flip(Plane(X,Y)),Rotate(Z,1)]
        };
        let test_ops = vec![twice_x,two_flip,flip_rotate];

        for point in test_points.iter() {
            for op in test_ops.iter() {
                let roundtrip = op.inverse(op.apply(*point));
                assert_eq!(*point, roundtrip);
            }
        }

    }

    // #[test]
    // fn test_chain_

    // #[test]
    // fn test_orientations() {
    //     let coord = Coord(5, 6, -4);
    //     let mut orients = orientations(coord);
    //     orients.sort();

    //     println!("{} total orientations", orients.len());

    //     let mut expected = vec![
    //         Coord(5, 6, -4),
    //         Coord(-5, 4, -6),
    //         Coord(4, 6, 5),
    //         Coord(-4, -6, 5),
    //         Coord(-6, -4, -5),
    //     ];
    //     expected.sort();
    //     assert_eq!(orients, expected);
    // }

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
