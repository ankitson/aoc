use std::collections::HashMap;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let lines = lines
            .iter()
            .map(|line| {
                let l = line.split('|').collect::<Vec<&str>>();
                let mut left = l[0].split_ascii_whitespace().collect::<Vec<&str>>();
                left.remove(left.len() - 1);
                // let leftsl = &left[0..left.len() - 1];
                // println!("left {:?}", left);
                let right = l[1].split_ascii_whitespace().collect::<Vec<&str>>();
                (left, right)
            })
            .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
        let mut count: usize = 0;
        for (_, second) in lines {
            let add = second.iter().filter(|x| vec![2, 4, 3, 7].contains(&x.len())).count();
            println!("adding {} for line {:?}", add, second);
            count += add
        }
        count
    }
}
