pub fn nbrs8(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let x = x as isize;
    let y = y as isize;
    let max_x = max_x as isize;
    let max_y = max_y as isize;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            } // Skip the cell itself

            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
                neighbors.push((nx as usize, ny as usize));
            }
        }
    }

    neighbors
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
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
