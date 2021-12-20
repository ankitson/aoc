use crate::shared::parse;
use itertools::Itertools;
use std::iter::repeat;

pub struct Soln1 {}
impl Soln1 {
    fn virtual_idx(grid: &Vec<Vec<usize>>, r: isize, c: isize, default: usize) -> usize {
        let gl: isize = grid.len().try_into().unwrap();
        if r >= 0 && r < gl && c >= 0 && c < gl {
            let ru: usize = r.try_into().unwrap();
            let cu: usize = c.try_into().unwrap();
            grid[ru][cu]
        } else {
            default
        }
    }

    fn mask_to_idx(mask: &mut dyn Iterator<Item = usize>) -> usize {
        let maske = mask.collect_vec();
        let bit_str: String = maske
            .iter()
            .map(|i| if (*i == 0) { '0' } else { '1' })
            .collect_vec()
            .into_iter()
            .collect();
        let map_idx: usize = usize::from_str_radix(&bit_str, 2).unwrap();
        map_idx
    }

    fn step(ehmap: &Vec<usize>, grid: &Vec<Vec<usize>>, default: usize) -> Vec<Vec<usize>> {
        let gl: isize = grid.len().try_into().unwrap();

        let mut new_grid = Vec::new();
        let newgl = gl + 2;
        let empty_row = repeat(0usize).take(newgl.try_into().unwrap()).collect_vec();
        (0..newgl).for_each(|i| new_grid.push(empty_row.clone()));
        for i in 0..newgl {
            for j in 0..newgl {
                let mut mask = vec![
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ]
                .into_iter()
                .map(|(r, c)| Self::virtual_idx(grid, r - 1, c - 1, default));
                let map_idx = Self::mask_to_idx(&mut mask);
                let iu: usize = i.try_into().unwrap();
                let ju: usize = j.try_into().unwrap();
                new_grid[iu][ju] = ehmap[map_idx];
            }
        }
        new_grid
    }

    fn step_n(ehmap: &Vec<usize>, grid: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
        let mut next = grid.clone();
        for i in 0..n {
            next = Self::step(&ehmap, &next, if i % 2 == 0 { 0 } else { 1 });
        }
        next
    }

    fn count_set(grid: &Vec<Vec<usize>>) -> usize {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn part1(input: &str) -> usize {
        let (ehmap, grid) = parse(input);
        let final_grid = Self::step_n(&ehmap, &grid, 2);

        Self::count_set(&final_grid)
    }

    pub fn part2(input: &str) -> usize {
        let (ehmap, grid) = parse(input);
        let final_grid = Self::step_n(&ehmap, &grid, 50);

        Self::count_set(&final_grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{parse, print_grid};

    use super::Soln1;

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let (ehmap, mut grid) = parse(sample);
        assert_eq!(Soln1::count_set(&Soln1::step_n(&ehmap, &grid, 1)), 24);
        assert_eq!(Soln1::count_set(&Soln1::step_n(&ehmap, &grid, 2)), 35);
    }

    #[test]
    fn test_visual_stepn() {
        let sample: &str = include_str!("../inputs/sample2.txt");
        let (ehmap, mut grid) = parse(sample);
        println!("grid:");
        print_grid(&grid);
        for i in 0..10 {
            let next = Soln1::step_n(&ehmap, &grid, i);
            println!("step:");
            print_grid(&next);
        }
    }
}
