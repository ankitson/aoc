pub struct Soln1 {}
impl Soln1 {
    pub fn parse(input: &str) -> Vec<i32> {
        input
            .split('\n')
            .map(|x| x.parse::<i32>().expect("Unable to parse int"))
            .collect()
    }

    pub fn unparse(output: i32) -> String {
        output.to_string()
    }

   pub fn part1(inputstr: &str) -> i32 {
       let input: Vec<&str> = inputstr.split('\n').collect();
       let mut max = 0;
        let mut current = 0;
        for i in 0..input.len() {
            let cals = input[i].parse::<i32>();
            println!("cals: {:?}", cals);
            match cals {
                Ok(i) => current += i,
                Err(_) => { 
                    if (current >= max) { 
                        max = current; 
                    } 
                    current = 0; 
                } 
            }
        }
        max        
    }

    pub fn part2(inputstr: &str) -> i32 {
       let input: Vec<&str> = inputstr.split('\n').collect();
       let mut max: Vec<i32> = vec![0,0,0] ;
		let mut current = 0;
        for i in 0..input.len() {
            let cals = input[i].parse::<i32>();
            println!("cals: {:?}", cals);
            match cals {
                Ok(i) => current += i,
                Err(_) => { 
					if (current > max[0]) {
						max[0] = current;
					}
					if (max[0] > max[1]) {
						let tmp = max[1];
						max[1] = max[0];
						max[0] = tmp;
					}
					if (max[1] > max[2]) {
						let tmp = max[2];
						max[2] = max[1];
						max[1] = tmp;
					}
                    current = 0; 
                } 
            }
        }
        max.iter().sum()
    }
}
