use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use regex::Regex;

pub type Input = (Vec<Vec<char>>, Vec<(usize, usize)>, Vec<usize>, Vec<usize>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid1 = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut galaxies = vec![];
    let mut extra_rows = vec![];
    let mut extra_cols = vec![];
    for i in 0..grid1.len() {
        let mut row_empty = true;
        for j in 0..grid1[i].len() {
            if grid1[i][j] == '#' {
                row_empty = false;
                galaxies.push((i, j));
            }
        }
        if row_empty {
            extra_rows.push(i);
        }
    }
    for j in 0..grid1[0].len() {
        let mut col_empty = true;
        for i in 0..grid1.len() {
            if grid1[i][j] == '#' {
                col_empty = false;
                break;
            }
        }
        if col_empty {
            extra_cols.push(j);
        }
    }
    (grid1, galaxies, extra_rows, extra_cols)
}

pub fn part1(raw_input: &str) -> Output {
    solve(raw_input, false)
}

pub fn solve(raw_input: &str, part2: bool) -> Output {
    let (grid, mut galaxies, er, ec) = parse(raw_input);
    let m = grid.len();
    let n = grid[0].len();
    let mut c = HashMap::new();
    // println!("m = {m} n = {n}");

    fn z(a: &usize, b: &usize, w: &usize) -> usize {
        let z = a * w + b;
        z
    }

    fn unz(z: &usize, w: &usize) -> (usize, usize) {
        let a = z / w;
        let b = z % w;
        (a, b)
    }
    galaxies.sort();
    let ztog = galaxies.iter().enumerate().map(|(i, (r, c))| (z(r, c, &m), i)).collect::<HashMap<_, _>>();

    for (g1, g2) in (0..galaxies.len()).tuple_combinations() {
        let (r1, c1) = galaxies[g1];
        let (r2, c2) = galaxies[g2];
        let smx = (r1.min(r2));
        let bgx = (r1.max(r2));
        let smy = (c1.min(c2));
        let bgy = (c1.max(c2));
        let dx = (r1 as isize - r2 as isize).abs() as usize;
        let dy = (c1 as isize - c2 as isize).abs() as usize;
        let mut d = dx + dy;

        let extra_row = (smx..bgx).filter(|i| er.contains(i)).count();
        let extra_col = (smy..bgy).filter(|i| ec.contains(i)).count();
        let mut extra = extra_row + extra_col;
        if part2 {
            extra *= (1000000 - 1);
        }
        d += extra;
        // let za = z(g1,, &m);
        // let zb = z(r2, c2, &m);
        c.insert((g1, g2), d);
    }

    // let cprint = c.iter().map(|((a, b), v)| format!("{}-{}: {}", a, b, v)).join("\n");
    // let numc = c.len();
    // println!("{numc} path values: {cprint}");

    let max_n = z(&m, &n, &m) - m;
    let mut npairs = 0;
    // let tot = max_n * max_n / 2 * max_n / 4;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            // let (ax, ay) = galaxies[i];
            // let (bx, by) = galaxies[j];
            // let gzi = z(&ax, &ay, &m);
            // let gzj = z(&bx, &by, &m);
            for k in 0..galaxies.len() {
                // let gzk = z(&galaxies[k].0, &galaxies[k].1, &m);

                npairs += 1;
                let dik = c.get(&(i, k)).or(c.get(&(k, i))).unwrap_or(&usize::MAX).clone();
                let dkj = c.get(&(k, j)).or(c.get(&(k, j))).unwrap_or(&usize::MAX).clone();
                if dik == usize::MAX || dkj == usize::MAX {
                    continue;
                }
                c.entry((i, j)).and_modify(|v| {
                    let new_dist = dik + dkj;
                    let curr_dist = *v;
                    let best_dist = new_dist.min(curr_dist);
                    *v = best_dist;
                });

                if npairs % 100000 == 0 {
                    // println!("{npairs}/{tot} pairs considered");
                }
            }
        }
    }
    // for i in 0..10 {
    // let g = &galaxies[i];
    // let zs = z(&galaxies[i].0, &galaxies[i].1, &m);
    // println!("galaxy {g:?} = {zs}");
    // }

    // 3 to 6 is one short, sum is 18 short
    // let cprint = c
    // .iter()
    // .sorted()
    // .map(|((a, b), v)| format!("{}-{}: {}", ztog.get(&a).unwrap() + 1, ztog.get(&b).unwrap() + 1, v))
    // .join("\n");
    // println!("path values: {cprint}");
    c.values().sum()
}

pub fn part2(raw_input: &str) -> Output {
    // let input = parse(raw_input);
    solve(raw_input, true)
    // todo!()
}
