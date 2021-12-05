use crate::aoc;

#[rustfmt::skip]
static input1: [&str; 12] = [
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
];

#[rustfmt::skip]
static INPUT2: [&str; 5] = [
    "10000",
    "01000",
    "00100",
    "00010",
    "00001"
];

// #[test]
fn test1() {
    let v1 = input1
        .into_iter()
        .map(|x| u32::from_str_radix(x, 2).expect("illegal int"))
        .collect::<Vec<u32>>();

    let (a, b) = aoc::day3::part1(&v1, 5);
    println!("a = {} b = {}", a, b);

    let (c, d) = aoc::day3::part2(&v1, 5);
    println!("c = {} d = {}", c, d);
}

#[test]
fn test2() {
    let v1 = INPUT2
        .into_iter()
        .map(|x| u32::from_str_radix(x, 2).expect("illegal int"))
        .collect::<Vec<u32>>();

    let (a, b) = aoc::day3::part1(&v1, 5);
    println!("a = {} b = {}", a, b);

    let (c, d) = aoc::day3::part2(&v1, 5);
    println!("c = {} d = {}", c, d);
}
