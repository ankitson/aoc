use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

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

pub fn part1_fast(raw_input: &str) -> Output {
    solve_fast(raw_input)
}

fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn grid_to_adjacency_matrix(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![0; grid.len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '#' {
                adj[i][j] = 1;
            }
        }
    }
    adj
}

pub fn solve(raw_input: &str, part2: bool) -> Output {
    let (_, galaxies, empty_rows, empty_cols) = parse(raw_input);
    let mut sum = 0;
    /*
    Becase all galaxies are directly connected to each other, with a consistent metric
    manhattan distance that follows triangle inequality, we dont need floyd-warshall
    d(a,b) = d(a,k) + d(k,b)
    */

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (r1, c1) = galaxies[i];
            let (r2, c2) = galaxies[j];
            let dx = (r1 as isize - r2 as isize).abs() as usize;
            let dy = (c1 as isize - c2 as isize).abs() as usize;
            let d = dx + dy;

            let smx = r1.min(r2);
            let bgx = r1.max(r2);
            let smy = c1.min(c2);
            let bgy = c1.max(c2);
            let extra_row = (smx..bgx).filter(|i| empty_rows.contains(i)).count();
            let extra_col = (smy..bgy).filter(|i| empty_cols.contains(i)).count();
            let mut extra = extra_row + extra_col;
            if part2 {
                extra *= 1000000 - 1;
            }

            sum += d + extra;
        }
    }
    sum
}

fn solve_fast(raw_input: &str) -> Output {
    /* 
    d(a,b) = |x1-x2| + |y1-y2| 
    dr(a,b) = |x1-x2| 
    dc(a,b) = |y1-y2|
    D = sum(d(a,b)) = sum(dr(a,b)) + sum(dc(a,b)) we can compute X & Y distances separately
    dc(yi) = (y9-yi) + (y8-yi) + (y7-yi) + (y6-yi) + (yi-yi-1) + (yi - yi-2) = (NCOLS - i) * (-yi) + i * yi + suffix[i:] - prefix[:i]
    */
    let (grid, mut galaxies, empty_rows, empty_cols) = parse(raw_input);
    
    let G = galaxies.len();
    let NROWS = grid.len();
    let NCOLS = grid[0].len();

    let mut prefix_x = vec![0isize; galaxies.len()];
    let mut suffix_x = vec![0isize; galaxies.len()];
    let mut prefix_y = vec![0isize; galaxies.len()];
    let mut suffix_y = vec![0isize; galaxies.len()];
    let (mut psx, mut psy, mut ssx, mut ssy) = (0, 0, 0, 0);
    galaxies.sort_by_key(|(y,x)| *x);
    println!("X sorted Galaxies: {:?}", galaxies);
    for i in 0..G {
        psx += galaxies[i].1 as isize;
        ssx += galaxies[G - i - 1].1 as isize;
        prefix_x[i] = psx;
        suffix_x[i] = ssx;
    }

    galaxies.sort_by_key(|(y,x)| *y);
    println!("Y sorted Galaxies: {:?}", galaxies);
    for i in 0..G {
        psy += galaxies[i].0 as isize;
        ssy += galaxies[G - i - 1].0 as isize;
        prefix_y[i] = psy;
        suffix_y[i] = ssy;
    }

    suffix_x.reverse();
    suffix_y.reverse();
    println!("X Prefixes: {:?}", prefix_x);
    println!("Y Prefixes: {:?}", prefix_y);
    println!("X Suffixes: {:?}", suffix_x);
    println!("Y Suffixes: {:?}", suffix_y);
    let mut sum = 0;

    galaxies.sort_by_key(|(y,x)| *y);
    for i in 0..G {
        let (yi, xi) = galaxies[i];
        let (yi, xi) = (yi as isize, xi as isize);
        let ii = i as isize;
        let NC = NCOLS as isize;
        let NR = NROWS as isize;
        
        //i=6, x=7,y=8
        #[rustfmt::skip]
        let sum_contrib_y: isize =
              ((G as isize -1  - ii) * -1isize * yi)
            + ii * yi * 1isize 
            + suffix_y[i]
            - prefix_y[i]; //suffix overcounts yi, prefix undercounts yi

        sum += sum_contrib_y;
        println!("galaxy {i} = {xi},{yi} contributes ysum {sum_contrib_y}");
    }
    galaxies.sort_by_key(|(y,x)| *x);
    for i in 0..G {
        let (yi, xi) = galaxies[i];
        let (yi, xi) = (yi as isize, xi as isize);
        let ii = i as isize;
        let NC = NCOLS as isize;
        let NR = NROWS as isize;
        
        let num_empty_cols_behind = empty_cols.iter().filter(|c| **c < xi as usize).count();
        let num_empty_cols_ahead = empty_cols.len() - num_empty_cols_behind;
        //i=6, x=7,y=8
        #[rustfmt::skip]
        let sum_contrib_x: isize =
              ((G as isize -1 - ii) * -1isize * xi) //3 * -1 * 7 = -21
            + ii * xi * 1isize //42
            + suffix_x[i] //11
            - prefix_x[i];
            // + (num_empty_cols_behind as isize) * ii
            // + (num_empty_cols_ahead as isize) * (G as isize - 1 - ii);

        sum += sum_contrib_x;
        println!("galaxy {i} = {xi},{yi} contributes xsum {sum_contrib_x}");
    }
    sum as usize
}
