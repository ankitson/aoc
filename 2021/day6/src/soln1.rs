pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str, days: usize) -> usize {
        let fish = input.trim().split(',').map(|x| x.parse::<u8>().expect("illegal age"));
        let mut fish = fish.collect::<Vec<u8>>();
        for _ in 0..days {
            Self::advance(&mut fish);
        }
        // println!("Final state: {:?}", fish);
        fish.len()
    }

    pub fn part2(input: &str, days: usize) -> u64 {
        let fish = input.trim().split(',').map(|x| x.parse::<u8>().expect("illegal age"));
        let mut by_age: [u64; 9] = [0; 9];
        for age in fish {
            by_age[usize::from(age)] += 1
        }
        //fish: 3,4,3,1,2
        //ba: [0,1,1,2,1,0,0,0,0]
        //ba: [1,1,2,1,0,0,0,0,0] == 0,1,2,2,3
        //ba: [1,2,1,0,0,0,1,0,1] == 0,1,1,2,6,8
        for day in 0..days {
            by_age[7] += by_age[0];
            by_age.rotate_left(1);
        }
        by_age.iter().sum()
    }

    fn advance(fish: &mut Vec<u8>) -> () {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] -= 1;
            }
        }
    }
}
