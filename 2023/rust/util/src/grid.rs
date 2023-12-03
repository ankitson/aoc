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
