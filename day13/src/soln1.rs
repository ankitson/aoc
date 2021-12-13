use itertools::Itertools;

use crate::shared::parse;
use crate::shared::{Fold, FoldType::XFold, FoldType::YFold};

pub struct Soln1 {}
impl Soln1 {
    pub fn fold(grid: &mut Vec<Vec<usize>>, fold: &Fold) -> (usize, usize) {
        let grid_size = grid.len();
        let mut max_x: usize = grid_size;
        let mut max_y: usize = grid_size;
        match fold {
            Fold { ftype: XFold, at } => {
                for y in 0..grid_size {
                    for x in 0..*at {
                        grid[y][x] += grid[y][2 * at - x];
                        grid[y][2 * at - x] = 0;
                    }
                }
                max_x = 2 * at;
            }
            Fold { ftype: YFold, at } => {
                for y in 0..*at {
                    for x in 0..grid_size {
                        grid[y][x] += grid[2 * at - y][x];
                        grid[2 * at - y][x] = 0;
                    }
                }
                max_y = 2 * at;
            }
        }
        (max_x, max_y)
    }

    fn str_grid(grid: &Vec<Vec<usize>>, max_row: usize, max_col: usize) -> String {
        let mut grid_str = String::new();
        for y in 0..=max_row {
            for x in 0..=max_col {
                if grid[y][x] == 0 {
                    grid_str.push('.');
                    // print!(".")
                } else {
                    grid_str.push('#');
                    // print!("#")
                }
            }
            grid_str.push('\n');
            // println!();
        }
        grid_str
    }

    pub fn part1(input: &str, grid_size: usize) -> usize {
        let (mut grid, folds) = parse(input, grid_size);

        let fold = &folds.into_iter().take(1).collect_vec()[0];
        Self::fold(&mut grid, fold);

        let mut dots: usize = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != 0 {
                    dots += 1
                }
            }
        }
        dots
    }
    pub fn part2(input: &str, grid_size: usize) -> String {
        let (mut grid, folds) = parse(input, grid_size);
        let mut max_x = grid_size;
        let mut max_y = grid_size;
        for fold in folds {
            let (fmax_x, fmax_y) = Self::fold(&mut grid, &fold);
            max_x = max_x.min(fmax_x);
            max_y = max_y.min(fmax_y);
        }
        Self::str_grid(&grid, max_y, max_x)
    }
}
