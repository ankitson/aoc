use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use util::grid;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    grid
}

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut t = vec![vec!['0'; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            t[j][i] = grid[i][j]
        }
    }
    t
}
fn score(grid: &Vec<Vec<char>>) -> usize {
    let mut nrows = grid.len();
    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        let rounded_rocks = row.iter().enumerate().filter(|(i, x)| **x == 'O').count();
        total += (nrows - i) * rounded_rocks;
    }
    total
}

fn score_rows(grid: &mut Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let ncols = grid.len();
    for row in grid {
        let mut slices = row.split_inclusive_mut(|x| *x == '#').collect_vec();
        for slice in &mut slices {
            slice.sort_by_key(|x| match *x {
                'O' => 0,
                '.' => 1,
                '#' => 2,
                _ => panic!("illegal"),
            });
        }
        let combined = slices.concat();
        let rounded_rocks = combined.iter().enumerate().filter(|(i, x)| **x == 'O');
        for (i, _) in rounded_rocks {
            total += ncols - i;
        }
    }
    total
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}
fn rotate_right(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut rotated = vec![vec!['X'; nrows]; ncols];
    for i in 0..nrows {
        for j in 0..ncols {
            rotated[j][nrows - 1 - i] = grid[i][j];
        }
    }
    rotated
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut grid = transpose(grid);
    score_rows(&mut grid)
}

fn roll_lr(grid: &mut Vec<Vec<char>>, left: bool) {
    for ri in 0..grid.len() {
        let row = &mut grid[ri];
        let mut slices = row.split_inclusive_mut(|x| *x == '#').collect_vec();
        for slice in &mut slices {
            slice.sort_by_key(|x| match *x {
                x if x != '.' && x != '#' => {
                    //should be 'O'
                    if left {
                        0
                    } else {
                        2
                    }
                }
                '.' => 1,
                '#' => {
                    if left {
                        2
                    } else {
                        2
                    }
                }
                _ => panic!("illegal"),
            });
        }
        let combined = slices.concat();
        grid[ri] = combined
    }
}

pub fn part2(raw_input: &str) -> Output {
    let mut grid = parse(raw_input);
    const NUM_ITERS: usize = 1000000000;

    /* let mut grid = vec![vec!['0'; 7]; 7];
    let mut idx = 0;
    for i in 0..7 {
        for j in 0..7 {
            grid[i][j] = (idx + '0' as u8) as char;
            idx += 1;
        }
    }
    grid[0][1] = '#';
    grid[0][2] = '.';
    grid[0][3] = '.';
    grid[1][2] = '#';
    grid[1][3] = '#';
    grid[1][4] = '.';
    grid[2][1] = '#';
    grid[2][3] = '.';
    grid[2][4] = '.';
    grid[4][4] = '#';
    grid[4][0] = '.'; */
    println!("grid: {grid:?}");
    print_grid(&grid);

    let mut seen = HashMap::new();
    let mut i = 0;
    // grid = transpose(grid);
    // println!("after trnaspoe");
    // print_grid(&grid);
    let mut skipped = false;
    /*
     1 2 3
     4 5 6
     7 8 9

     ROLL FROM 7->1
     ROLL FROM 3->1
     ROLL FROM 1->7
     ROLL FROM 1->3

     transpose =
     1 4 7
     2 5 8
     3 6 9

     R1=
     7 8 9
     4 5 6
     1 2 3

     R2=
     9 6 3
     8 5 2
     7 4 1

     R3=
     3 2 1
     6 5 4
     9 8 7

     R4=orig
     1 4 7
     2 5 8
     3 6 9

     roll_left(transpose(grid)) = ROLL FROM 7->1 = roll_up(grid)
     roll_right(R3(transpose(grid))) = ROLL FROM 3->1 = roll_west(grid)
     roll_right(transpose(grid)) = ROLL FROM 1->7 = roll_down(grid)
     roll_left(R3(transpose(grid))) = ROLL FROM 1->3 = roll_east(grid

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
        grid = rotate_right(grid);
        roll_lr(&mut grid, false); //N
                                   // println!("after roll 1");
                                   // print_grid(&grid);

        grid = rotate_right(grid);
        roll_lr(&mut grid, false); //W
                                   // println!("after roll 2");
                                   // print_grid(&grid);

        grid = rotate_right(grid);
        roll_lr(&mut grid, false); //S
                                   // println!("after roll 3");
                                   // print_grid(&grid);

        grid = rotate_right(grid);
        roll_lr(&mut grid, false); //E
        println!("after cycle {i}");
        print_grid(&grid); //at this point, grid is back to orig

        // AFTER ROTATION NUMBER i=9 (0 indexed so 10)
        // GRID MATCHES AFTER ROTATION NUMBER 2 (3)
        // HENCE CYCLE LEN = 7
        // NUM_ITERS = 20
        // REM_ITERS = 20-9-1 = 10
        // (10 / 7) = 1 cycle of length 7 remains
        // i += 1*7. i = 16
        // i += 12*7 = 84
        // I = 12+84 = 96

        if !skipped {
            if let Some(&last_idx) = seen.get(&grid) {
                println!("last = {last_idx} i = {i}");
                let cycle_len = i - last_idx;
                let rem_iters = NUM_ITERS - i - 1;
                let fwd = (rem_iters / cycle_len) * cycle_len;
                i += fwd;
                skipped = true;
            } else {
                let current_score = score(&grid);
                seen.insert(grid.clone(), i);
                println!("score = {current_score}");
                // let orig_grid = transpose(grid.clone());
                // print_grid(&grid);
                println!("------------------------------------")
            }
        }
        i += 1;
    }
    println!("FINAL GRID:");
    print_grid(&grid);
    let score = score(&mut grid);
    score

    // for _ in 0..NUM_ITERS {
    // grid = rotate_right(grid);
    // }
    // let rotated = rotate_right(grid);
    // println!("rotated:");
    // print_grid(&rotated);
    // todo!()
}
