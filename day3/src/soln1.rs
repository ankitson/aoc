use std::collections::HashSet;

use itertools::Itertools;

pub struct Soln1 {}
impl Soln1 {
 
    pub fn part1(input: &str) -> i32 {
        let lines = input.split('\n').collect_vec();
        let mut total = 0;
        for line in lines {
            let size = line.len() / 2;
            let cm = Self::common_characters(&line[0..size], &line[size..line.len()]);

            let score: i32 = cm.iter().map(|x| Self::priority(x)).sum();
            total += score;
        }
        total
    }

    //ChatGPT
    fn common_characters(s1: &str, s2: &str) -> Vec<char> {
        let mut common = vec![];
        let set1: HashSet<char> = s1.chars().collect();
        let set2: HashSet<char> = s2.chars().collect();

        for c in set1.intersection(&set2) {
            common.push(*c);
        }

        common
    }

    fn priority(ch: &char) -> i32 {
        if (ch.is_ascii_uppercase()) {
            ((*ch as i32) - ('A' as i32)) + 27
        } else {
            ((*ch as i32) - ('a' as i32)) + 1
        }
    }

    pub fn part2(input: &str) -> i32 {
        let lines = input.split('\n').collect_vec();
        let mut total = 0;
        for chunk in lines.chunks(3) {
            if (chunk.len() < 3) {
                continue;
            }
            let cm1 = Self::common_characters(chunk[0], chunk[1]);
            let cm1s: String = cm1.iter().collect();
            let cm = Self::common_characters(
                    &cm1s,
                    chunk[2]
            );
            total += Self::priority(&cm[0]);            
        }
        total
    }
}
