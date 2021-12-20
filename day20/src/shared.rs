use itertools::Itertools;

pub fn parse(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let enhance_map = lines.next().unwrap().chars().map(|c| { if c == '.' { 0} else { 1 }}).collect_vec();

    lines.next();

    let mut grid:Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let nums = line.chars().map(|c| { if (c == '.') { 0 } else { 1 } });
        grid.push(nums.collect_vec());
    }

    (enhance_map, grid)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        let (ehmap, grid) = parse(include_str!("../inputs/sample.txt"));
        assert_eq!(ehmap.len(), 512);
        assert_eq!(grid.len(), 5);
        assert!(grid.iter().all(|r| r.len() == 5));

        let (ehmap, grid) = parse(include_str!("../inputs/day20.txt"));
        assert_eq!(ehmap.len(), 512);
        assert_eq!(grid.len(), 100);
        assert!(grid.iter().all(|r| r.len() == 100));
    }
}
