use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use regex::Regex;

pub type U3 = (usize, usize, usize);
pub type Input = Vec<(U3, U3)>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let bricks = input
        .lines()
        .map(|line| {
            let (starts, ends) = line.split_once("~").unwrap();
            let scs = starts.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
            let (sx, sy, sz) = (scs[0], scs[1], scs[2]);
            let ecs = ends.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
            let (ex, ey, ez) = (ecs[0], ecs[1], ecs[2]);
            ((sx, sy, sz), (ex, ey, ez))
        })
        .collect_vec();
    bricks
}

// fn overlaps(i1: (usize, usize), i2: (usize, usize)) -> bool {
//     // -------
//     //    ------
//     println!("i1 = {:?}, i2 = {:?}", i1, i2);
//     let (a, b) = i1;
//     let (c, d) = i2;

//     let mut sms = 0;
//     let mut sme = 0;
//     let mut bgs = 0;
//     let mut bge = 0;
//     if a <= c {
//         sms = a.min(b);
//         sme = b.max(a);
//         bgs = c.min(d);
//         bge = d.max(c);
//     } else {
//         sms = c.min(d);
//         sme = d.max(c);
//         bgs = a.min(b);
//         bge = b.max(a);
//     }

//     if sme < bgs {
//         println!("\tfalse");
//         return false;
//     }
//     println!("\ttrue");
//     return true;
// }

// pub fn part1(raw_input: &str) -> Output {
//     let bricks = parse(raw_input);
//     let mut sorted_zs = bricks
//         .iter()
//         .cloned()
//         .map(|((a, b, c), (d, e, f))| if c < f { ((a, b, c), (d, e, f)) } else { ((d, e, f), (a, b, c)) })
//         .collect_vec();
//     sorted_zs.sort_by_key(|((a, b, c), (d, e, f))| *c);

//     /*
//     Brick A cannot be disintegrated safely; if it were disintegrated, bricks B and C would both fall.
//     Brick B can be disintegrated; the bricks above it (D and E) would still be supported by brick C.
//     Brick C can be disintegrated; the bricks above it (D and E) would still be supported by brick B.
//     Brick D can be disintegrated; the brick above it (F) would still be supported by brick E.
//     Brick E can be disintegrated; the brick above it (F) would still be supported by brick D.
//     Brick F cannot be disintegrated; the brick above it (G) would fall.
//     Brick G can be disintegrated; it does not support any other bricks.
//      */
//     /* This fails because we haven't simulated the bricks falling yet.
//     Brick C (#2) is at the same height as Brick B (#1) AFTER falling but not before */
//     let mut num_remove = 0;
//     for (zi, brick) in sorted_zs.iter().enumerate() {
//         let mut can_remove = false;
//         let mut num_supporting = 0;
//         let mut next_zi = zi + 1;

//         if zi > 0 {
//             let prev_zi = zi - 1;
//             let prev_z = sorted_zs[prev_zi].1 .2;
//             if prev_z == brick.0 .2 {
//                 can_remove = true;
//                 num_remove += 1;
//                 println!("Can remove brick {zi} because it has an earlier Z neighbor = {brick:?}");
//                 continue;
//             }
//         }

//         let mut next_z = 0;
//         if next_zi < sorted_zs.len() {
//             next_z = sorted_zs[next_zi].1 .2;
//         }
//         if next_z == brick.0 .2 {
//             can_remove = true;
//             num_remove += 1;
//             println!("Can remove brick {zi} because it has a later Z neighbor = {brick:?}");
//             continue;
//         }
//         if next_zi == sorted_zs.len() {
//             println!("Can remove brick {zi} because it is the last brick= {brick:?}");
//             can_remove = true;
//             num_remove += 1;
//             continue;
//         }
//         let next_higher = sorted_zs[next_zi];
//         println!("check if brick {zi} {brick:?} overlaps with {next_higher:?}");
//         let ((sx, sy, sz), (ex, ey, ez)) = brick;
//         let ((sx2, sy2, sz2), (ex2, ey2, ez2)) = next_higher;

//         let x_overlaps = overlaps((*sx, *ex), (sx2, ex2));
//         let y_overlaps = overlaps((*sy, *ey), (sy2, ey2));
//         if !(x_overlaps && y_overlaps) {
//             println!("Can remove brick {zi} because iet doesnt overlap with above= {brick:?}");
//             can_remove = true;
//             num_remove += 1;
//         }
//     }
//     num_remove
// }
fn dropped(tallest: &HashMap<(usize, usize), usize>, brick: &mut (U3, U3)) -> usize {
    let peak = *(brick.0 .0..=brick.1 .0)
        .map(|x| (brick.0 .1..=brick.1 .1).map(|y| tallest.get(&(x, y)).unwrap_or(&0)).max().unwrap_or(&0))
        .max()
        .unwrap_or(&0);
    let fall = (brick.0 .2 as i32 - peak as i32 - 1).max(0) as usize;
    brick.0 .2 -= fall as usize;
    brick.1 .2 -= fall as usize;
    fall
}

fn fall_all(bricks: &mut Vec<(U3, U3)>) -> usize {
    let mut tallest = HashMap::new();
    let mut falls = 0;
    for i in 0..bricks.len() {
        let f = dropped(&tallest, &mut bricks[i]);
        falls += if f > 0 { 1 } else { 0 };
        for x in bricks[i].0 .0..=bricks[i].1 .0 {
            for y in bricks[i].0 .1..=bricks[i].1 .1 {
                tallest.insert((x, y), bricks[i].1 .2);
            }
        }
    }
    falls
}

/** assume bricks below this one have fallen already, bricks must be a sorted slice of bricks below this one **/
// fn fall(brick: (U3, U3), bricks: &[(U3, U3)]) -> usize {
//     let mut fall = 0;
//     let ((sx, sy, sz), (ex, ey, ez)) = brick;
//     let mut curr_zs = sz;
//     for below in bricks.iter().rev() {
//         let ((bsx, bsy, bsz), (bex, bey, bez)) = below;
//         if !(overlaps((sz, ez), (*bsz, *bez)) || overlaps((sx, ez), (*bsx, *bex)) || overlaps((sy, ey), (*bsy, *bey))) {
//             //(overlaps((sx, ex), (*bsx, *bex)) && overlaps((sy, ey), (*bsy, *bey))) {
//             fall += curr_zs - bez;
//             println!("Brick {brick:?} falls by {fall} because it doesnt overlap with {below:?}");
//             curr_zs -= fall;
//         } else {
//             break;
//         }
//     }
//     fall
// }

pub fn part1(raw_input: &str) -> Output {
    let mut bricks = parse(raw_input);
    bricks.sort_by_key(|((a, b, c), (d, e, f))| *c);

    fall_all(&mut bricks);

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut bricks_without = bricks.clone();
        bricks_without.remove(i);
        let nfalls = fall_all(&mut bricks_without);
        // println!("Brick {i} falls {nfalls}");
        total += if nfalls == 0 { 1 } else { 0 };
    }
    total
}

// pub fn part1_new(raw_input: &str) -> Output {
//     let mut bricks = parse(raw_input);
//     let mut sorted_zs = bricks
//         .iter()
//         .cloned()
//         .map(|((a, b, c), (d, e, f))| if c < f { ((a, b, c), (d, e, f)) } else { ((d, e, f), (a, b, c)) })
//         .collect_vec();
//     sorted_zs.sort_by_key(|((a, b, c), (d, e, f))| *c);

//     //Assert: all bricks are straight lines
//     for &brick in &bricks {
//         let ((sx, sy, sz), (ex, ey, ez)) = brick;
//         assert!(
//             (sx == ex && sy == ey && sz != ez)
//                 || (sx == ex && sy != ey && sz == ez)
//                 || (sx != ex && sy == ey && sz == ez)
//         );
//     }

//     // Simulate falling for all bricks
//     for i in 0..bricks.len() {
//         let brick = bricks[i];
//         let falls = fall(brick, &bricks[..i]);
//         println!("Brick {i} falls by {falls:?}");
//         let mut new_brick = brick.clone();
//         new_brick.0 .2 -= falls;
//         new_brick.1 .2 -= falls;
//         bricks[i] = new_brick;
//     }

//     println!("After falling: bricks = {:?}", bricks);

//     /*
//     Brick A cannot be disintegrated safely; if it were disintegrated, bricks B and C would both fall.
//     Brick B can be disintegrated; the bricks above it (D and E) would still be supported by brick C.
//     Brick C can be disintegrated; the bricks above it (D and E) would still be supported by brick B.
//     Brick D can be disintegrated; the brick above it (F) would still be supported by brick E.
//     Brick E can be disintegrated; the brick above it (F) would still be supported by brick D.
//     Brick F cannot be disintegrated; the brick above it (G) would fall.
//     Brick G can be disintegrated; it does not support any other bricks.
//      */
//     // any brick can be supported under any of its (x,y) coords.
//     //vert bricks only have one (x,y) coord
//     let mut brick_map: BTreeMap<((usize, usize, usize), (usize, usize, usize)), Vec<(usize, usize, usize)>> =
//         BTreeMap::new();
//     for &brick in &bricks {
//         let ((sx, sy, sz), (ex, ey, ez)) = brick;
//         brick_map.insert(brick, vec![]);
//         // let e = brick_map.entry(brick);
//         // let mut entry = brick_map
//         for x in sx..=ex {
//             for y in sy..=ey {
//                 for z in sz..ez {
//                     brick_map.entry(brick).or_insert(vec![]).push((x, y, z));
//                 }
//             }
//         }
//     }

//     let mut count = 0;
//     for &brick in &bricks {
//         let ((sx, sy, sz), (ex, ey, ez)) = brick;
//         let mut can_remove = false;
//         let blocks = brick_map.get(&brick).unwrap();
//         todo!()
//     }

//     todo!()
// }

pub fn part2(raw_input: &str) -> Output {
    let mut bricks = parse(raw_input);
    bricks.sort_by_key(|((a, b, c), (d, e, f))| *c);

    fall_all(&mut bricks);

    let mut total = 0;
    for i in 0..bricks.len() {
        let mut bricks_without = bricks.clone();
        bricks_without.remove(i);
        let nfalls = fall_all(&mut bricks_without);
        total += nfalls;
    }
    total
}
