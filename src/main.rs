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
    println!("part 1 answer = {}", u32::from(a) * u32::from(b)); //3923414
    let (c, d) = aoc::day3::part2(&nums, bit_width);
    println!("oxygen = {} co2 = {}", c, d);
    println!("part 2 answer = {}", u32::from(c) * u32::from(d)); //5852595

    println!("---Day 4---");
    let (moves, boards) = aoc::day4::input();
    println!("moves: {:?}\n board: {:?}", moves, boards[0]);
    let (sum, winning_num) = aoc::day4::part1(moves, boards);
    println!("sum: {} winning num: {}", sum, winning_num);
    println!("part 1 answer = {}", sum * winning_num); //49860

    let (moves, boards) = aoc::day4::input();
    //println!("moves: {:?}\n board: {:?}", moves, boards[0]);
    let (sum, winning_num) = aoc::day4::part2(moves, boards);
    println!("sum: {} winning num: {}", sum, winning_num);
    println!("part 2 answer = {}", sum * winning_num); //49860
}
