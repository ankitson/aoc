use shared;

pub struct Soln1 {}
impl Soln1 {
    fn count_window(input: Vec<i32>, window_size: usize) -> i32 {
        let mut count = 0;
        for i in 0..input.len() - window_size {
            count += if input[i + window_size] > input[i] { 1 } else { 0 };
        }
        count
    }
}

impl shared::Solution<Vec<i32>, i32> for Soln1 {
    fn parse(input: &str) -> Vec<i32> {
        input
            .split('\n')
            .map(|x| x.parse::<i32>().expect("Unable to parse int"))
            .collect()
    }

    fn unparse(output: i32) -> String {
        output.to_string()
    }

    fn part1_core(input: Vec<i32>) -> i32 {
        Self::count_window(input, 1)
    }

    fn part2_core(input: Vec<i32>) -> i32 {
        Self::count_window(input, 2)
    }
}
