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

    println!("Hello Day 12!");
    let part1s = vec![
        ("sample1", include_str!("../../inputs/sample12.txt"), 140),
        ("day12", include_str!("../../inputs/day12.txt"), 1465112),
    ];
    for (name, input, expected) in &part1s {
        let result = soln1::part1(input);
        println!("part1/{name} = {result:?} ?= {expected}");
        assert_eq!(result, *expected);
    }

    let part2s = vec![
        ("sample1", include_str!("../../inputs/sample12.txt"), 0),
        // ("day12", include_str!("../../inputs/day12.txt"), 0),
    ];
    for (name, input, expected) in &part2s {
        let result = soln1::part2(input);
        println!("part2/{name} = {result:?} ?= {expected}");
        assert_eq!(result, *expected);
    }
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
