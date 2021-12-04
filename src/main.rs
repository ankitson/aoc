mod aoc;
mod tests;

fn main() {
    println!("Hello, world!");

    println!("---Day 1---");
    println!("{}", aoc::day1::part1()); //1387
    println!("{}", aoc::day1::part2()); //1362

    println!("---Day 2---");
    println!("{}", aoc::day2::part1()); //1728414
    println!("{}", aoc::day2::part2()); //1765720035

    println!("---Day 3---");
    let (nums_iter, bit_width) = aoc::day3::input();
    let nums = nums_iter.collect::<Vec<u32>>();
    let (a, b) = aoc::day3::part1(&nums, bit_width);
    println!("gamma = {} epsilon = {}", a, b);
    println!("part 1 answer = {}", aoc::day3::widen_mul(a, b)); //3923414
    let (c, d) = aoc::day3::part2(&nums, bit_width);
    println!("oxygen = {} co2 = {}", c, d);
    println!("part 2 answer = {}", aoc::day3::widen_mul(c, d)); //5852595
}
