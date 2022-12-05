mod soln1;

pub fn main() {
    println!("Hello World");
    let contents: &str = include_str!("../inputs/day3.txt");
    let (input_iter, input_width) = soln1::Soln1::parse(contents);
    let nums = input_iter.collect::<Vec<u32>>();
    let part1 = soln1::Soln1::part1(&nums, input_width);
    let part1_out = soln1::Soln1::unparse(part1);
    println!("Part 1 = {}", part1_out);

    let part2 = soln1::Soln1::part2(&nums, input_width);
    let part2_out = soln1::Soln1::unparse(part2);
    println!("Part 2 = {}", part2_out);
}
