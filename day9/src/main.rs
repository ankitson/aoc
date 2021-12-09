#![feature(drain_filter)]
mod soln1;

pub fn main() {
    println!("Hello Day 9!");
    let contents: &str = include_str!("../inputs/part1.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 = {:?}", part1);
}

pub mod shared {
    pub fn parse(input: &str) -> Vec<Vec<u8>> {
        let parsed = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (c.to_digit(10).unwrap() as u8))
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        println!("{:?}", &parsed);
        parsed
    }
}

#[cfg(test)]
mod tests {}
