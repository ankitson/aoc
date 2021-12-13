use itertools::Itertools;

// mod shared;
// use crate::shared::Fold;
use crate::shared::parse;
use crate::shared::{Fold, FoldType::XFold, FoldType::YFold};

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let (mut grid, folds) = parse(input);

        let folds = folds.into_iter().take(1).collect_vec();

        for fold in folds {
            println!("processing fold {:?}", fold);
            match fold {
                Fold { ftype: XFold, at } => {
                    //[0,1,2,3,4,5,6]
                    //fold at x=3
                    //4 -> 2, 5 -> 1, 6 -> 0
                    //(fx+i) -> (fx-i)
                    for y in 0..100 {
                        for x in 0..at {
                            grid[y][x] += grid[y][x + at];
                            grid[y][x + at] = 0;
                        }
                    }
                }
                Fold { ftype: YFold, at } => {
                    println!("yfold at {}", at);
                    for y in 0..at {
                        for x in 0..100 {
                            grid[y][x] += grid[y + at][x];
                            grid[y + at][x] = 0;
                        }
                    }
                }
            }
        }

        let mut dots: usize = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != 0 {
                    println!("dot: {}, {}", x, y);
                    dots += 1
                }
            }
        }
        dots
    }
}
