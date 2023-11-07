use crate::shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> i32 {
        Self::part1_core(shared::parse(input))
    }

    pub fn part1_core(grid: Vec<Vec<u32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut num_visible = 0;
        for i in 0..rows {
            for j in 0..cols {
                let is_blocked = (Self::blocked(-1, 0, i, j, &grid)
                    && Self::blocked(1, 0, i, j, &grid)
                    && Self::blocked(0, 1, i, j, &grid)
                    && Self::blocked(0, -1, i, j, &grid));
                if !is_blocked {
                    num_visible += 1;
                }
            }
        }
        num_visible
    }

    fn blocked(dx: i32, dy: i32, x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut rx: i32 = (i32::try_from(x).unwrap() + dx);
        let mut ry: i32 = (i32::try_from(y).unwrap() + dy);
        while rx >= 0 && ry >= 0 && rx < rows.try_into().unwrap() && ry < cols.try_into().unwrap() {
            let xx = usize::try_from(rx).unwrap();
            let yy = usize::try_from(ry).unwrap();
            if grid[xx][yy] >= grid[x][y] {
                return true;
            }
            rx = rx + dx;
            ry = ry + dy;
        }
        false
    }

    pub fn part2(input: &str) -> i32 {
        Self::part2_core(shared::parse(input))
    }

    pub fn part2_core(grid: Vec<Vec<u32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut max_score = 0;
        for i in 0..rows {
            for j in 0..cols {
                let score = (Self::score(-1, 0, i, j, &grid)
                    * Self::score(1, 0, i, j, &grid)
                    * Self::score(0, 1, i, j, &grid)
                    * Self::score(0, -1, i, j, &grid));
                max_score = max_score.max(score);
            }
        }
        max_score
    }

    fn score(dx: i32, dy: i32, x: usize, y: usize, grid: &Vec<Vec<u32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut rx: i32 = (i32::try_from(x).unwrap() + dx);
        let mut ry: i32 = (i32::try_from(y).unwrap() + dy);
        let mut score = 0;
        while rx >= 0 && ry >= 0 && rx < rows.try_into().unwrap() && ry < cols.try_into().unwrap() {
            let xx = usize::try_from(rx).unwrap();
            let yy = usize::try_from(ry).unwrap();
            if grid[xx][yy] < grid[x][y] {
                score += 1
            }
            if grid[xx][yy] >= grid[x][y] {
                score += 1;
                break;
            }
            rx = rx + dx;
            ry = ry + dy;
        }
        score
    }
}
