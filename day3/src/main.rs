mod soln1;

pub fn main() {
    println!("Hello Day 3!");
    let input: &str = include_str!("../inputs/sample3.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Sample = {}", part1); //157

    let input: &str = include_str!("../inputs/day3.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Input 1 = {}", part1); //7990

    let input: &str = include_str!("../inputs/day3.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("Part 2 / Input 1 = {}", part2); //2602
}
