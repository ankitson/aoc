fn parse() -> Vec<i32> {
    let contents: &str = include_str!("../../input/day1.txt");
    contents
        .split('\n')
        .map(|x| x.parse::<i32>().expect("Unable to parse int"))
        .collect()
}

fn count_window(input: Vec<i32>, window_size: usize) -> i32 {
    let mut count = 0;
    for i in 0..input.len() - window_size {
        count += if input[i + window_size] > input[i] { 1 } else { 0 };
    }
    count
}

pub fn part1() -> String {
    count_window(parse(), 1).to_string()
}

pub fn part2() -> String {
    count_window(parse(), 3).to_string()
}
