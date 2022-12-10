use itertools::Itertools;

pub struct Soln1 {}
impl Soln1 {

    pub fn part1(input: &str) -> i32 {
        Self::part1_core(input)
    }

    pub fn part1_core(input: &str) -> i32 {
        let cycles = [20,60,100,140,180,220];
        let mut cycle = 1;
        let mut x = 1;
        let mut strength = 0;
        for line in input.lines() {
            let parts = line.split(" ").collect_vec();
            match parts[0] {
                "noop" => {
                    if cycles.contains(&cycle) {
                        strength += cycle * x;
                    } 
                    cycle += 1;
                },
                "addx" => {
                    if cycles.contains(&cycle) {
                        strength += cycle * x;
                    } else if cycles.contains(&(cycle+1)) {
                        strength += (cycle+1) * x;
                    }
                    cycle += 2;
                    let n = parts[1].parse::<i32>().expect("ah");
                    x += n;
                },
                _      => panic!("illegal")
            }
        }
        strength
    }

    pub fn part2(input: &str) -> () {
        Self::part2_core(input)
    }

    pub fn part2_core(input: &str) -> () {
        /*
         * The cycle num = pixel currently drawing on
         * [x-1,x,x+1] -> the 3 pixels that are on
         * 
         * if cycle num in [x-1,x,x+1] -> draw on pixel
         */
        let mut screen = vec!['.'; 40*6];
        let mut cycle: usize = 1;
        let mut x = 1i32;
        for line in input.lines() {
            let parts = line.split(" ").collect_vec();

            match parts[0] {
                "noop" => {
                    //Value of X here = Value before/during CYCLE
                    //cycle is 1-indexed.
                    //we draw pixel CYCLE-1 during CYCLE
                    let vert_pos = ((cycle-1) % 40) as i32;
                    if vec![x-1,x,x+1].contains(&vert_pos) {
                        screen[cycle-1] = '#';
                    }
                    cycle += 1;
                    //after cycle 1
                },
                "addx" => {
                    //Value of X here = Value before/during CYCLE
                    let vert_pos = ((cycle-1) % 40) as i32;
                    if vec![x-1,x,x+1].contains(&vert_pos) {
                        screen[cycle-1] = '#';
                    }

                    //Value of X here = Value before/during CYCLE+1
                    let vert_pos = ((cycle-1+1) % 40) as i32;
                    if vec![x-1,x,x+1].contains(&(vert_pos)) {
                        screen[cycle] = '#';
                    }

                    cycle += 2;
                    let n = parts[1].parse::<i32>().expect("ah");
                    x += n;
                },
                _      => panic!("illegal")
            }
        }
        for i in 0..40*6 {
            print!("{}",screen[i]);
            if i % 40 == 39 {
                println!();
            }
        }
        ()        
    }


}
