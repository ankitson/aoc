#![feature(drain_filter)]
mod soln1;
mod soln2;

pub fn main() {
    println!("Hello Day 8!");
    let contents: &str = include_str!("../inputs/day8.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1: {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    println!("Part 2: {:?}", part2);
    // let part2 = soln::Soln2::part2(contents);
    // println!("Part 2 (constraint propagation) = {:?}", part2);
    // let part1 = soln::Soln1::part1_fast(contents);
    // println!("Part 1 (quickselect+median): {:?}", part1);
    // let part2 = soln::Soln1::part2_fast(contents);
    // println!("Part 2 (mean): {:?}", part2);
}



#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{soln1, soln2};

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample8.txt");
        let part1 = soln1::Soln1::part1(sample);
        println!("Part 1: {}", part1);
        assert_eq!(part1, 26);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample8.txt");
        let answer = soln1::Soln1::part2(sample);
        println!("Part 2 = {:?}", answer);
        assert_eq!(answer, 61229);
        // let part2 = soln::Soln2::part2(contents);
        // println!("Part 2 (constraint propagation) = {:?}", part2);
    }

    #[test]
    fn test_part2_constraint_prop() {
        let sample: &str = include_str!("../inputs/sample8.txt");
        let answer = soln2::Soln2::part2(sample.split('\n').next().unwrap());
        println!("Part 2 (constraint prop) = {:?}", answer);
        // assert_eq!(answer, 61229);
        // let part2 = soln::Soln2::part2(contents);
        // println!("Part 2 (constraint propagation) = {:?}", part2);
    }

    #[test]
    fn test_part2_merge_maps() {
        let mut m1: HashMap<char, HashSet<char>> = HashMap::from_iter([('a', HashSet::from_iter(['a', 'b', 'c']))]);
        let m2: HashMap<char, HashSet<char>> = HashMap::from_iter([('a', HashSet::from_iter(['b', 'c', 'd']))]);

        let merged: HashMap<char, HashSet<char>> = HashMap::from_iter([('a', HashSet::from_iter(['b', 'c']))]);
        soln2::Soln2::merge_maps(&mut m1, m2);
        assert_eq!(m1, merged);
    }
}
