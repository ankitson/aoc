mod shared;
mod soln1;

#[allow(unused_imports)]
#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    let sample1: &str = include_str!("../../inputs/sample11.txt");
    let sample2: &str = include_str!("../../inputs/sample11_2.txt");
    let real: &str = include_str!("../../inputs/day11.txt");
    let inps = [("sample1", sample1), ("sample2", sample2), ("real", real)];
    println!("Hello Day 11!");
    for (label, inp) in inps {
        let part1 = soln1::part1(inp);
        let part1_fast = soln1::part1_fast(inp);
        let part1_rec = soln1::part1_rec(inp);
        println!("part1/{label} = {part1:?}");
        println!("part1_fast/{label} = {part1_fast:?}");
        println!("part1_rec/{label} = {part1_rec:?}");

        let part2 = soln1::part2(inp);
        println!("part2/{label} = {part2:?}");
    }

    // let part1 = soln1::part1(input);
    // println!("part1/sample2 = {:?}", part1);
    // let part1 = soln1::part1_fast(input);
    // println!("part1_fast/sample2 = {:?}", part1);

    // let part1 = soln1::part1(input);
    // println!("part1/day11 = {:?}", part1);
    // let part1 = soln1::part1_fast(input);
    // println!("part1_fast/day11 = {:?}", part1);

    // let input: &str = include_str!("../../inputs/sample11.txt");
    // let part2 = soln1::part2(input);
    // println!("part2/sample11 = {:?}", part2);

    // let input: &str = include_str!("../../inputs/day11.txt");
    // let part2 = soln1::part2(input);
    // println!("part2/day11 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::soln1;

    #[test]
    fn test_part1() {
        println!("Testing part1...");
        assert!(1 == 1)
    }
}
