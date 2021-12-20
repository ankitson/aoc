use itertools::Itertools;

pub fn parse(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let enhance_map = lines.next().unwrap().chars().map(|c| { if c == '.' { 0} else { 1 }}).collect_vec();

    lines.next();

    let mut grid:Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let mut row = line.chars().map(|c| { if c == '.' { 0 } else { 1 } }).collect_vec();
        grid.push(row);
    }

    (enhance_map, grid)
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<usize>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ch = { if grid[i][j] == 0 { '.' } else {'#'} };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        let (ehmap, grid) = parse(include_str!("../inputs/sample.txt"));
        assert_eq!(ehmap.len(), 512);
        assert_eq!(grid.len(), 5);
        assert!(grid.iter().all(|c| c.len() == 5));

        let (ehmap, grid) = parse(include_str!("../inputs/day20.txt"));
        assert_eq!(ehmap.len(), 512);
        assert_eq!(grid.len(), 100);
        assert!(grid.iter().all(|r| r.len() == 100));
    }
}
