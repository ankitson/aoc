#[path = "shared.rs"]
mod shared;

fn color_ansi(color: &str) -> &str {
    match color {
        "black" => &"\x001b[30m",
        "red" => &"\x001b[31m",
        "green" => &"\x001b[32m",
        "yellow" => &"\x001b[33m",
        "blue" => &"\x001b[34m",
        "magenta" => &"\x001b[35m",
        "cyan" => &"\x001b[36m",
        "white" => &"\x001b[37m",
        "reset" => &"\x001b[0m",
        _ => panic!("unknown color"),
    }
}

pub struct Soln1 {}
impl Soln1 {
    fn alloc<T>(grid: &Vec<Vec<T>>) {
        let rows = grid.len();
        let cols = grid[0].len();

        for _ in 0..rows {
            println!("{}", "0".repeat(cols));
        }
        print!("\x001b[{}A", rows);
        print!("\x001b[{}D", cols);
    }

    pub fn part1(input: &str, nsteps: usize) -> u64 {
        let mut grid = shared::parse(input);
        Self::alloc(&grid);
        let mut flashes = 0;
        for _ in 0..nsteps {
            flashes += Self::step(&mut grid);
        }
        flashes
    }

    pub fn part2(input: &str) -> Option<usize> {
        let mut grid = shared::parse(input);
        let nrows: u64 = grid.len().try_into().unwrap();
        let ncols: u64 = grid[0].len().try_into().unwrap();

        let mut i = 0;
        loop {
            let step_flashes = Self::step(&mut grid);
            if (step_flashes == nrows * ncols) {
                return Some(i + 1);
            }
            i += 1;
        }
        unreachable!()
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> u64 {
        let rows = grid.len();
        let cols = grid[0].len();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                print!("{:.1}", grid[i][j]);
                grid[i][j] += 1
            }
        }
        print!("\x001b[{}A", rows);
        print!("\x001b[{}D", cols);

        let mut flashed: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];
        let mut quiet = false;
        let mut nflashes: u64 = 0;
        while !quiet {
            quiet = true;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if (grid[i][j] > 9) {
                        for (nbrx, nbry) in Self::nbrs(i, j, &grid) {
                            if flashed[nbrx][nbry] != 1 {
                                grid[nbrx][nbry] += 1;
                            }
                        }
                        grid[i][j] = 0;
                        flashed[i][j] = 1;
                        quiet = false;
                        nflashes += 1;
                    }
                }
            }
        }
        nflashes
    }

    fn print_grid(grid: &Vec<Vec<u32>>) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                print!("{}", grid[i][j]);
            }
            println!();
        }
        println!();
    }

    fn nbrs(x: usize, y: usize, hts: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
        let max_x = hts.len();
        let max_y = hts[0].len();
        let cands = vec![
            (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() - 1),
            (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() + 0),
            (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() + 1),
            (i64::try_from(x).unwrap() + 0, i64::try_from(y).unwrap() - 1),
            (i64::try_from(x).unwrap() + 0, i64::try_from(y).unwrap() + 1),
            (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() - 1),
            (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() + 0),
            (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() + 1),
        ];

        let mut nbrs2: Vec<(usize, usize)> = Vec::new();
        for (a, b) in cands {
            if a < 0 || b < 0 {
                continue;
            }
            let aa = a.try_into().unwrap();
            let bb = b.try_into().unwrap();
            if aa < max_x && bb < max_y {
                nbrs2.push((aa, bb));
            }
        }
        nbrs2
    }
}
