#![feature(drain_filter)]
mod soln1;

pub fn main() {
    println!("Hello Day 9!");
    let contents: &str = include_str!("../inputs/sample.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part 1 = {:?}", part1);
    let part2 = soln1::Soln1::part2(contents);
    let product: usize = part2.iter().product();
    println!("Part 2 = {:?} = {}", part2, product);

    let part2 = soln1::Soln1::part2_mut(contents);
    let product: usize = part2.iter().product();
    println!("Part 2 (mutate) = {:?} = {}", part2, product);
}

#[cfg(test)]
mod tests {}
