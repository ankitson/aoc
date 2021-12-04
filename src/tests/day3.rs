use crate::aoc;

#[test]
fn test1() {
    #[rustfmt::skip]
    let test_input = [
        "00100", 
        "11110", 
        "10110", 
        "10111", 
        "10101", 
        "01111", 
        "00111", 
        "11100", 
        "10000", 
        "11001", 
        "00010", 
        "01010",
    ]
    .into_iter()
    .map(|x| u32::from_str_radix(x, 2).expect("illegal int"));

    let (a, b) = aoc::day3::part1(&test_input.collect::<Vec<u32>>(), 5);
    println!("a = {} b = {}", a, b);
}
