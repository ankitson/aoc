mod soln1;

pub fn main() {
    println!("Hello Day 3!");
    let input: &str = include_str!("../inputs/sample3.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Sample = {}", part1);

    let input: &str = include_str!("../inputs/day3.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Input 1 = {}", part1);

    // let part2 = soln1::Soln1::part2(&nums, input_width);
    // let part2_out = soln1::Soln1::unparse(part2);
    // println!("Part 2 = {}", part2_out);
}
