mod soln1;

pub fn main() {
    println!("Hello Day 2!");
    let contents: &str = include_str!("../../inputs/sample02.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part1/Sample = {:?}", part1);

    let contents: &str = include_str!("../../inputs/day02.txt");
    let part1 = soln1::Soln1::part1(contents);
    println!("Part1/Input1 = {:?}", part1); //11222

    let contents: &str = include_str!("../../inputs/sample02.txt");
    let part2 = soln1::Soln1::part2(contents);
    println!("Part2/Sample = {:?}", part2); //11222

    let contents: &str = include_str!("../../inputs/day02.txt");
    let part2 = soln1::Soln1::part2(contents);
    println!("Part2/Input1 = {:?}", part2); //11222

    //let part1 = soln1::Soln1::part1_core(input);
    //let part1_out = soln1::Soln1::unparse(part1);
    //println!("Part 1 = {}", part1_out);

    //  let input = soln1::Soln1::parse(contents);
    //  let part2 = soln1::Soln1::part2_core(input);
    //  let part2_out = soln1::Soln1::unparse(part2);
    //  println!("Part 2 = {}", part2_out);
}
