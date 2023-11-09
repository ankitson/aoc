mod soln1;

pub fn main() {
    println!("Hello Day 4!");
    let input: &str = include_str!("../../inputs/sample4.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part1/Sample = {:?}", part1); //2

    let input: &str = include_str!("../../inputs/day4.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part1/Input1 = {:?}", part1); //580

    let input: &str = include_str!("../../inputs/sample4.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("Part2/Sample = {:?}", part2); //2

    let input: &str = include_str!("../../inputs/day4.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("Part2/Input1 = {:?}", part2); //895
}
