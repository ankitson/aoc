use itertools::Itertools;

pub type Input = (Vec<Vec<usize>>, (usize, usize), (usize, usize));
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let grid = input.split_whitespace().map(|row| row.chars().collect_vec()).collect_vec();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    let mut num_grid: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    for rnum in 0..grid.len() {
        for cnum in 0..grid[rnum].len() {
            if grid[rnum][cnum] == 'S' {
                start = Some((rnum, cnum));
                num_grid[rnum][cnum] = 0
            } else if grid[rnum][cnum] == 'E' {
                end = Some((rnum, cnum));
                num_grid[rnum][cnum] = 25;
            } else {
                num_grid[rnum][cnum] = (grid[rnum][cnum] as usize) - ('a' as usize)
            }
        }
    }
    (num_grid, start.unwrap(), end.unwrap())
}
