use std::collections::{HashMap, HashSet};

pub struct Soln1 {}
impl Soln1 {
    fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let lines = lines
            .iter()
            .map(|line| {
                let l = line.split('|').collect::<Vec<&str>>();
                let mut left = l[0].split_ascii_whitespace().collect::<Vec<&str>>();
                left.remove(left.len() - 1);
                let right = l[1].split_ascii_whitespace().collect::<Vec<&str>>();
                (left, right)
            })
            .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
        lines
    }
    pub fn part1(input: &str) -> usize {
        let lines = Soln1::parse(input);
        let mut count: usize = 0;
        for (_, second) in lines {
            let add = second.iter().filter(|x| vec![2, 4, 3, 7].contains(&x.len())).count();
            println!("adding {} for line {:?}", add, second);
            count += add
        }
        count
    }

    /*
    lets number the segments
    *  0000
    * 1    2
    * 1    2
    *  3333
    * 4    5
    * 4    5
    *  6666
    *
    * candidate = mapping from letter to segment
    * (a-f) -> (0-6)
    *
    * 7! mappings to start with
    * then filter
    *
    * mapping format
    * segment
    */

    pub fn part2(input: &str) -> usize {
        let lines = Soln1::parse(input);

        let mut candidates: Vec<Vec<usize>> = vec![];
        Soln1::seed_candidates(&mut candidates, (0..7).collect(), vec![0; 7]);
        println!("{:?}", candidates);
        5
    }

    //BROKEN: stack overflows
    fn seed_candidates(vec: &mut Vec<Vec<usize>>, remaining: HashSet<usize>, mut candidate: Vec<usize>) {
        if (remaining.len() == 1) {
            candidate[0] = *remaining.iter().next().unwrap();
            vec.push(candidate);
            return;
        }
        //generate all permutations of 0-6
        for i in 0..7 {
            let mut copy = candidate.clone();
            copy[0] = i;
            let mut remaining_copy = remaining.clone();
            remaining_copy.remove(&i);
            Soln1::seed_candidates(vec, remaining_copy, copy);
        }
    }
}
