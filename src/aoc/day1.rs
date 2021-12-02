fn parse() -> Vec<i32> {
    let contents: &str = include_str!("../../input/day1.txt");
    contents
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn part1() -> String {
    let nums = parse();

    let mut i = 1;
    let mut prev: i32 = nums[0];
    let mut incr = 0;
    while i < nums.len() {
        if nums[i] > prev {
            incr += 1;
        }
        prev = nums[i];
        i += 1;
    }
    incr.to_string()
}

pub fn part2() -> String {
    let nums = parse();

    let mut iter = nums.windows(3);
    let mut prev: i32 = iter.next().unwrap().iter().sum();
    let mut incr = 0;

    for window in iter {
        let sum: i32 = window.iter().sum();
        if sum > prev {
            incr += 1
        }
        prev = sum;
    }

    incr.to_string()
}
