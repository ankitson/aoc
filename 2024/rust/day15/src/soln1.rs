use itertools::Itertools;
use regex::Regex;
use util::grid::{self, print_grid};

pub type Input = (Vec<Vec<char>>, Vec<char>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let (gridstr, movestr) = input.split_once("\n\n").unwrap();
    let grid = gridstr.trim().lines().map(|line| line.chars().collect_vec()).collect_vec();
    let moves = movestr.trim().chars().filter(|c| !c.is_whitespace()).collect_vec();
    (grid, moves)
}

fn mov_to_dir(mov: char) -> (isize, isize) {
    match mov {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => panic!("illegal move: {mov}"),
    }
}

fn step(grid: &mut Vec<Vec<char>>, mov: char, sr: usize, sc: usize) -> (usize, usize) {
    let (dr, dc) = mov_to_dir(mov);
    let sr: isize = sr as isize;
    let sc: isize = sc as isize;
    let (nr, nc) = (sr as isize + dr, sc as isize + dc);
    println!("mov {mov} sr,sc = {sr},{sc}, nr,nc = {nr},{nc}");
    if (nr == sr) && (nc == sc) {
        return (sr as usize, sc as usize);
    }
    if !(nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid[0].len() as isize) {
        return (sr as usize, sc as usize);
    }
    if grid[nr as usize][nc as usize] == '#' {
        return (sr as usize, sc as usize);
    }
    if grid[nr as usize][nc as usize] == '.' {
        println!("can move to nbr, no obstruction");
        grid[sr as usize][sc as usize] = '.';
        grid[nr as usize][nc as usize] = '@';
        return (nr as usize, nc as usize);
    }

    let mut curr = '@';
    let (mut pr, mut pc) = (nr, nc);
    println!("finding blank");
    while pr >= 0
        && pr < grid.len() as isize
        && pc >= 0
        && pc < grid[0].len() as isize
        && grid[pr as usize][pc as usize] == 'O'
    {
        println!("pr = {pr} pc = {pc}");
        pr += dr;
        pc += dc;
    }
    if grid[pr as usize][pc as usize] == '#' {
        return (sr as usize, sc as usize);
    }
    println!("moving blocks");
    while !(pr == nr && pc == nc) {
        println!("pr = {pr} pc = {pc}");
        grid[pr as usize][pc as usize] = grid[(pr - dr) as usize][(pc - dc) as usize];
        grid[(pr - dr) as usize][(pc - dc) as usize] = '.';
        pr = pr - dr;
        pc = pc - dc;
    }
    grid[sr as usize][sc as usize] = '.';
    grid[nr as usize][nc as usize] = '@';
    ((sr + dr) as usize, (sc + dc) as usize)
}
pub fn part1(raw_input: &str) -> Output {
    let (mut grid, moves) = parse(raw_input);
    let (sr, sc) = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == '@').map(|col| (row, col)))
        .unwrap();
    let (mut cr, mut cc) = (sr, sc);
    println!("Grid {grid:?} moves = {moves:?} robot={sr},{sc}");
    // panic!("exit");
    for mov in &moves {
        println!("-------- move={mov}");
        (cr, cc) = step(&mut grid, *mov, cr, cc);
        print_grid(&grid);
        println!("--------")
    }
    let mut total = 0;
    for (ri, row) in grid.iter().enumerate() {
        for (ci, item) in row.iter().enumerate() {
            if *item == 'O' {
                total += 100 * (ri) + ci
            }
        }
    }
    total
}

pub fn part2(raw_input: &str) -> Output {
    let input = parse(raw_input);
    todo!()
}
