pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> () {
        let nums = input
            .trim()
            .split(',')
            .map(|x| x.parse::<u8>().expect("illegal int"))
            .collect::<Vec<u8>>();
        println!("{:?}", nums);

        
        ()
    }
}
