mod soln1;

pub fn main() {
    println!("Hello Day 3!");
    let input: &str = include_str!("../inputs/sample4.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("Part 1 / Sample = {:?}", part1); //2

    let input: &str = include_str!("../inputs/day4.txt");
    let (overlaps1, count1) = soln1::Soln1::part1_wrong(input);
    let (overlaps2, count2) = soln1::Soln1::part1(input);

    //this reveals the bug: when both intervals have the same left endpoint,
    //they will not be swapped. we assume r1 contains r2, but r2 may be the bigger one. 
    println!("the wrong implementation missed {} overlaps", count2-count1);
    for item in &overlaps2 {
        if !(overlaps1.contains(item)) {
            println!("{:?}", item);
        }
    }


    // println!("Part 1 / Input 1 = {:?}", part1); //580

    // let input: &str = include_str!("../inputs/day4.txt");
    // let part2 = soln1::Soln1::part2(input);
    // println!("Part 2 / Input 1 = {:?}", part2); //895
}
