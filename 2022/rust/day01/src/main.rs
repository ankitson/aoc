mod soln1;

pub fn main() {
    println!("Hello Day 1!");
    let contents: &str = include_str!("../../inputs/sample1.txt");
    let part1 = soln1::Soln1::part1(contents);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Part1/Sample = {}", part1_out);

    let contents: &str = include_str!("../../inputs/day1.txt");
    let part1 = soln1::Soln1::part1(contents);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Part1/Input1 = {}", part1_out);

    let contents: &str = include_str!("../../inputs/sample1.txt");
    let part2 = soln1::Soln1::part2(contents);
    let part2_out = soln1::Soln1::unparse(part2);
    println!("Part2/Sample = {}", part2_out);

    let contents: &str = include_str!("../../inputs/day1.txt");
    let part2 = soln1::Soln1::part2(contents);
    let part2_out = soln1::Soln1::unparse(part2);
    println!("Part2/Input1 = {}", part2_out);
}
