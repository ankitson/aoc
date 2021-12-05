mod soln;
use shared::Solution;

pub fn main() {
    println!("Hello World");
    let contents: &str = include_str!("../day2.txt");
    let input = soln::Soln1::parse(contents);
    let part1 = soln::Soln1::part1_core(input);
    let part1_out = soln::Soln1::unparse(part1);
    println!("Part 1 = {}", part1_out);

    let input = soln::Soln1::parse(contents);
    let part2 = soln::Soln1::part2_core(input);
    let part2_out = soln::Soln1::unparse(part2);
    println!("Part 2 = {}", part2_out);
}