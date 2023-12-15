use itertools::Itertools;
use std::collections::HashMap;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    grid
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let nrows = grid.len();
    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        let rounded_rocks = row.iter().enumerate().filter(|(i, x)| **x == 'O').count();
        total += (nrows - i) * rounded_rocks;
    }
    total
}

fn rotate(grid: Vec<Vec<char>>, cw: bool) -> Vec<Vec<char>> {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut rotated = vec![vec!['X'; nrows]; ncols];
    for i in 0..nrows {
        for j in 0..ncols {
            if cw {
                rotated[j][nrows - 1 - i] = grid[i][j];
            } else {
                rotated[ncols - 1 - j][i] = grid[i][j];
            }
        }
    }
    rotated
}

fn roll_lr(grid: &mut Vec<Vec<char>>, left: bool) {
    for ri in 0..grid.len() {
        let row = &mut grid[ri];
        let mut slices = row.split_inclusive_mut(|x| *x == '#').collect_vec();
        for slice in &mut slices {
            slice.sort_by_key(|x| match *x {
                'O' => (1 - (left as u8)) * 2,
                '.' => 1,
                '#' => 2,
                _ => panic!("illegal"),
            });
        }
        let combined = slices.concat();
        grid[ri] = combined
    }
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut grid = rotate(grid, true);
    roll_lr(&mut grid, false);
    let grid = rotate(grid, false);
    score(&grid)
}

pub fn part2(raw_input: &str) -> Output {
    let mut grid = parse(raw_input);
    const NUM_ITERS: usize = 1000000000;
    let mut seen = HashMap::new();
    let mut i = 0;
    let mut skipped = false;
    /*
     1 2 3
     4 5 6
     7 8 9

     ROLL FROM 7->1
     ROLL FROM 3->1
     ROLL FROM 1->7
     ROLL FROM 1->3

        N
       W E
        S

        W
       S N
        E

        S
       E W
        N

        E
       N S
        W
    */

    while i < NUM_ITERS {
        grid = rotate(grid, true);
        roll_lr(&mut grid, false); //N

        grid = rotate(grid, true);
        roll_lr(&mut grid, false); //W

        grid = rotate(grid, true);
        roll_lr(&mut grid, false); //S

        grid = rotate(grid, true);
        roll_lr(&mut grid, false); //E

        /*
        say we find grid after rot i=9 (0 indexed) matches grid after rot i=2
        cycle len = 9-2 = 7
        rem_iters = total_iters - i - 1 = 20-9-1 = 10
        rem_cycles = (10 / 7) = 1 cycle of length 7 remains
        increment i += (rem_cycles)*cycle_len = 1*7. i=16.
        */
        if !skipped {
            if let Some(&last_idx) = seen.get(&grid) {
                let cycle_len = i - last_idx;
                let rem_iters = NUM_ITERS - i - 1;
                let fwd = (rem_iters / cycle_len) * cycle_len;
                i += fwd;
                skipped = true;
            } else {
                seen.insert(grid.clone(), i);
            }
        }
        i += 1;
    }
    let score = score(&mut grid);
    score
}
