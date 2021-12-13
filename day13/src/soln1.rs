use itertools::Itertools;

// mod shared;
// use crate::shared::Fold;
use crate::shared::parse;
use crate::shared::{Fold, FoldType::XFold, FoldType::YFold};

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str, grid_size: usize) -> usize {
        let (mut grid, folds) = parse(input, grid_size);

        let folds = folds.into_iter().take(1).collect_vec();

        for fold in folds {
            println!("processing fold {:?}", fold);
            match fold {
                Fold { ftype: XFold, at } => {
                    //[0,1,2,3,4,5,6]
                    //fold at x=3
                    //4 -> 2, 5 -> 1, 6 -> 0
                    //(fx+i) -> (fx-i)
                    //in math terms:
                    for y in 0..grid_size {
                        for x in 0..at {
                            grid[y][x] += grid[y][2 * at - x];
                            grid[y][2 * at - x] = 0;
                        }
                    }
                }
                Fold { ftype: YFold, at } => {
                    println!("yfold at {}", at);
                    for y in 0..at {
                        for x in 0..grid_size {
                            grid[y][x] += grid[2 * at - y][x];
                            grid[2 * at - y][x] = 0;
                        }
                    }
                }
            }
        }

        let mut dots: usize = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != 0 {
                    // print!("#");
                    // println!("dot: {}, {}", x, y);
                    dots += 1
                } else {
                    // print!(".");
                }
            }
            // println!();
        }
        dots
    }
}
