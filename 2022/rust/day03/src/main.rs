mod soln1;

pub fn main() {
    println!("Hello Day 3!");
    let input: &str = include_str!("../../inputs/sample03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part1/Sample = {}", part1); //157

    let input: &str = include_str!("../../inputs/day03.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part1/Input1 = {}", part1); //7990

    let input: &str = include_str!("../../inputs/sample03.txt");
    let part2 = soln1::Soln1::part1(input);
    println!("Part2/Sample = {}", part2); //7990

    let input: &str = include_str!("../../inputs/day03.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("Part2/Input1 = {}", part2); //2602
}
