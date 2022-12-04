mod soln1;

pub fn main() {
    println!("Hello Day 1!");
    let contents: &str = include_str!("../inputs/sample1.txt");
    let part1 = soln1::Soln1::part1(contents);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Sample 1 = {}", part1_out);

    let contents: &str = include_str!("../inputs/day1.txt");
    let part1 = soln1::Soln1::part1(contents);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Part 1 = {}", part1_out);

//    let input = soln1::Soln1::parse(contents);
//    let part2 = soln1::Soln1::part2_core(input);
//    let part2_out = soln1::Soln1::unparse(part2);
//    println!("Part 2 = {}", part2_out);

}
