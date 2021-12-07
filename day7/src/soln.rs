use std::collections::HashMap;

pub struct Soln1 {}
impl Soln1 {
    fn parse(input: &str) -> Vec<i32> {
        let nums = input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().expect("illegal int"))
            .collect::<Vec<i32>>();
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
        let mut masses = HashMap::new();
        for crab in &nums {
            let idx = (crab - min) as usize;
            let key = masses.entry(idx).or_insert(0);
            *key += 1;
        }
        let mut min_fuel = i32::MAX;
        for pos in *min..*max + 1 {
            let fuel = Self::fuel_part2(&nums, pos);
            min_fuel = min_fuel.min(fuel)
        }
        min_fuel
    }

    pub fn median_quickselect(k: usize, mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        // note below does not work, because the drain_filter may remove or mutate nums[0]
        // let less = nums.drain_filter(|x| x <= &mut nums[0]);
        let pivot = &mut nums[0].clone();

        let less = nums.drain_filter(|x| x < pivot).collect::<Vec<i32>>();
        let eq = nums.drain_filter(|x| x == pivot).collect::<Vec<i32>>();
        let eqlen = eq.len();
        let greater = nums;

        if k < less.len() {
            Soln1::median_quickselect(k, less)
        } else if k < less.len() + eqlen {
            *pivot
        } else {
            Soln1::median_quickselect(k - less.len() - eqlen, greater)
        }
    }

    pub fn part1_fast(input: &str) -> i32 {
        let nums = Soln1::parse(input);
        let nums_copy = nums.clone();
        let median = Soln1::median_quickselect(nums.len() / 2, nums);
        Self::fuel_used(&nums_copy, median)
    }

    pub fn part2_fast(input: &str) -> i32 {
        let nums = Soln1::parse(input);
        let mut mean: f64 = f64::try_from(nums.iter().sum::<i32>()).expect("overflow");
        mean = mean / f64::try_from(nums.len() as i32).expect("overflow");
        let cand1 = mean.trunc() as i32;
        let cand2 = mean.trunc() as i32 + 1;
        let cand3 = mean.trunc() as i32 - 1;

        //error: this doesnt compile either
        // let cands = [cand1, cand2, cand3];
        // let min = cands.map(|p| Soln1::fuel_part2(&nums, p)).iter().min().unwrap();
        // *min

        let fuel1 = Soln1::fuel_part2(&nums, cand1);
        let fuel2 = Soln1::fuel_part2(&nums, cand2);
        let fuel3 = Soln1::fuel_part2(&nums, cand3);
        fuel1.min(fuel2).min(fuel3)
    }

    fn fuel_part2(positions: &Vec<i32>, at: i32) -> i32 {
        positions
            .iter()
            .map(|p| {
                let diff = (p - at).abs();
                diff * (diff + 1) / 2
            })
            .sum()
    }

    fn fuel_used(positions: &Vec<i32>, at: i32) -> i32 {
        positions.iter().map(|p| (p - at).abs()).sum()
    }
}
