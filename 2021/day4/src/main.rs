mod soln1;

pub fn main() {
    let contents: &str = include_str!("../inputs/day4.txt");
    let (moves, boards) = soln1::Soln1::parse(contents);
    let part1 = soln1::Soln1::part1(moves, boards);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Part 1 = {}", part1_out);

    let (moves, boards) = soln1::Soln1::parse(contents);
    let part2 = soln1::Soln1::part2(moves, boards);
    let part2_out = soln1::Soln1::unparse(part2);
    println!("Part 2 = {}", part2_out);
}
