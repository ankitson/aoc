#[path = "shared.rs"]
mod shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str, nsteps: usize) -> u64 {
        let mut grid = shared::parse(input);
        let mut flashes = 0;
        for _ in 0..nsteps {
            flashes += Self::step(&mut grid).0;
            Self::print_grid(&grid);
        }
        flashes
    }

    pub fn part2(input: &str) -> Option<usize> {
        let mut grid = shared::parse(input);
        let mut flashes = 0;
        let mut i = 0;
        while true {
            let allflash = Self::step(&mut grid).1;
            if (allflash) {
                return Some(i + 1);
            }
            i += 1;
        }
        return None;
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> (u64, bool) {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                grid[i][j] += 1
            }
        }

        let mut flashed: Vec<Vec<u32>> =
            vec![vec![0; grid[0].len()]; grid.len()];
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

        // println!("step after");
        // Self::print_grid(&grid);
        // println!("flashed after");
        // Self::print_grid(&flashed);
        let mut all_flash = true;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if flashed[i][j] == 0 {
                    all_flash = false;
                }
            }
        }

        (nflashes, all_flash)
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
        let mut nbrs: Vec<(i64, i64)> = Vec::new();
        nbrs = vec![
            (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() - 1),
            (
                i64::try_from(x).unwrap() - 1,
                i64::try_from(y).unwrap() + 0i64,
            ),
            (i64::try_from(x).unwrap() - 1, i64::try_from(y).unwrap() + 1),
            (
                i64::try_from(x).unwrap() + 0i64,
                i64::try_from(y).unwrap() - 1,
            ), /*(xy    */
            (
                i64::try_from(x).unwrap() + 0i64,
                i64::try_from(y).unwrap() + 1,
            ),
            (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() - 1),
            (
                i64::try_from(x).unwrap() + 1,
                i64::try_from(y).unwrap() + 0i64,
            ),
            (i64::try_from(x).unwrap() + 1, i64::try_from(y).unwrap() + 1),
        ];

        let mut nbrs2: Vec<(usize, usize)> = Vec::new();
        for (a, b) in nbrs {
            if (a < 0 || b < 0) {
                continue;
            }
            let aa = a.try_into().unwrap();
            let bb = b.try_into().unwrap();
            if (aa < max_x && bb < max_y) {
                nbrs2.push((aa, bb));
            }
        }
        nbrs2
    }
}
