use std::collections::HashMap;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let lines = lines
            .chunks(2)
            .map(|l| {
                let mut left = l[0].split_ascii_whitespace().collect::<Vec<&str>>();
                left.remove(left.len() - 1);
                // let leftsl = &left[0..left.len() - 1];
                println!("left {:?}", left);
                let right = l[1].split_ascii_whitespace().collect::<Vec<&str>>();
                (left, right)
            })
            .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
        println!("{:?}", lines);
    }
}
