use itertools::Itertools;
use std::collections::HashMap;

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

pub fn part2(raw_input: &str) -> Output {
    solve(raw_input, true)
}

pub fn solve(raw_input: &str, part2: bool) -> Output {
    let (_, galaxies, empty_rows, empty_cols) = parse(raw_input);
    let mut c = HashMap::new();
    for (g1, g2) in (0..galaxies.len()).tuple_combinations() {
        let (r1, c1) = galaxies[g1];
        let (r2, c2) = galaxies[g2];
        let smx = r1.min(r2);
        let bgx = r1.max(r2);
        let smy = c1.min(c2);
        let bgy = c1.max(c2);
        let dx = (r1 as isize - r2 as isize).abs() as usize;
        let dy = (c1 as isize - c2 as isize).abs() as usize;
        let mut d = dx + dy;

        let extra_row = (smx..bgx).filter(|i| empty_rows.contains(i)).count();
        let extra_col = (smy..bgy).filter(|i| empty_cols.contains(i)).count();
        let mut extra = extra_row + extra_col;
        if part2 {
            extra *= 1000000 - 1;
        }
        d += extra;
        c.insert((g1, g2), d);
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            for k in 0..galaxies.len() {
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
            }
        }
    }
    c.values().sum()
}
