use itertools::Itertools;

use crate::shared::{parse, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);

        //x indexes columns left to right, y indexes rows top to bottom

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut max_dist = i32::MIN;
        for &((sx, sy), (bx, by)) in input.iter() {
            min_x = sx.min(bx.min(min_x));
            min_y = sy.min(by.min(min_y));
            max_x = sx.max(bx.max(max_x));
            max_y = sy.max(by.max(max_y));

            let dx: i32 = sx.abs_diff(bx).try_into().unwrap();
            let dy: i32 = sy.abs_diff(by).try_into().unwrap();
            max_dist = (dx + dy).max(max_dist);
        }
        // println!("x bound: 0 -> ({} - {} + {})", max_x, min_x, max_dist);
        // println!("y bound: 0 -> ({} - {} + {})", max_y, min_y, max_dist);
        let grid_width: usize = (max_x - min_x + 1 + max_dist)
            .try_into()
            .expect(format!("illegal grid X bounds {} {}", min_x, max_x).as_str());
        let grid_height: usize = (max_y - min_y + 1 + max_dist)
            .try_into()
            .expect(format!("illegal grid Y bounds {} {}", min_y, max_y).as_str());
        let mut grid = vec![vec![0; grid_height]; grid_width];
        for &((sx, sy), (bx, by)) in input.iter() {
            let (sxx, syy) = Self::g(sx, sy, min_x, min_y);
            let (bxx, byy) = Self::g(bx, by, min_x, min_y);
            grid[sxx][syy] = 1;
            grid[bxx][byy] = 2;
            let dx = sxx.abs_diff(bxx);
            let dy = syy.abs_diff(byy);
            let min = -1 * ((dx + dy) as i32);
            let max = ((dx + dy) as i32);
            println!("for sensor at {}, {}, the beacon dist is {} + {} = {}", sx, sy, dx, dy, dx + dy);
            for xo in min..=max {
                let yo = ((dx + dy) as i32) - xo;
                // let (nx, ny) =
                let (nx, ny) = Self::g((sx as i32) + xo, (sy as i32) + yo, min_x + min, min_y + min);
                if (nx < grid.len()) && (ny < grid[0].len()) {
                    println!("marking point {}, {} as covered", nx, ny);
                    grid[nx][ny] = 3;
                }
            }
        }
        Self::print_grid(&grid);
        let count = grid.iter().map(|c| if (c[10] == 3) { 1 } else { 0 }).sum();
        count
        // Self::part1_core(&input)
    }

    fn orient(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trans = vec![vec![0; grid.len()]; grid[0].len()];
        for ci in 0..grid.len() {
            for ri in 0..grid[0].len() {
                trans[ri][ci] = grid[ci][ri];
            }
        }
        trans
    }

    fn print_grid(grid_in: &Vec<Vec<i32>>) {
        let grid = Self::orient(grid_in);
        for col in grid.iter() {
            for item in col.iter() {
                if (*item == 0) {
                    print!(".")
                } else if (*item == 1) {
                    print!("S")
                } else if (*item == 2) {
                    print!("B")
                } else if (*item == 3) {
                    print!("#")
                } else {
                    panic!("illegal grid num")
                }
                // print!("{}", col);
            }
            println!("")
        }
    }

    fn g(x: i32, y: i32, min_x: i32, min_y: i32) -> (usize, usize) {
        // println!("x y mx my {} {} {} {}", x, y, min_x, min_y);
        let xx: usize = (x - min_x).try_into().unwrap();
        let yy: usize = (y - min_y).try_into().unwrap();
        (xx, yy)
    }

    pub fn part1_core(input: &Input) -> Output {
        todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
