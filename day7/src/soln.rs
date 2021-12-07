pub struct Soln1 {}
impl Soln1 {
    fn parse(input: &str) -> Vec<i32> {
        let nums = input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().expect("illegal int"))
            .collect::<Vec<i32>>();
        dbg!("{:?}", &nums);
        nums
    }

    pub fn part1(input: &str) -> i32 {
        let nums = Soln1::parse(input);
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        let mut min_fuel = i32::MAX;
        for pos in *min..*max + 1 {
            let fuel = Self::fuel_used(&nums, pos);
            min_fuel = min_fuel.min(fuel)
        }
        min_fuel
    }

    pub fn part2(input: &str) -> i32 {
        let nums = Soln1::parse(input);
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        let mut min_fuel = i32::MAX;
        for pos in *min..*max + 1 {
            let fuel = Self::fuel_part2(&nums, pos);
            min_fuel = min_fuel.min(fuel)
        }
        min_fuel
    }

    fn fuel_part2(positions: &Vec<i32>, at: i32) -> i32 {
        //pos diff = n
        //fuel usage = 1 + 2 + .. n = n*(n-1)/2
        positions
            .iter()
            .map(|p| {
                let diff = (p - at).abs();
                if diff <= 1 {
                    diff
                } else {
                    diff * (diff + 1) / 2
                }
            })
            .sum()
    }

    fn fuel_used(positions: &Vec<i32>, at: i32) -> i32 {
        positions.iter().map(|p| (p - at).abs()).sum()
    }
}
