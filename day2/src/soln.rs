use shared;

#[derive(Debug)]
pub enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
    Empty,
}

pub struct Soln1 {}
impl Soln1 {
    pub fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
        input.split('\n').map(|line| {
            if line.is_empty() {
                return Instruction::Empty;
            }
            let mut parts = line.split_ascii_whitespace();
            let dir = parts.next().expect("illegal line");
            let amount = parts
                .next()
                .expect("illegal line")
                .parse::<u32>()
                .expect("illegal line");

            match dir {
                "forward" => Instruction::Forward(amount),
                "down" => Instruction::Down(amount),
                "up" => Instruction::Up(amount),
                _ => panic!("illegal instruction"),
            }
        })
    }

    pub fn unparse(output: (u32, u32)) -> String {
        (output.0 * output.1).to_string()
    }

    pub fn part1_core(input: impl Iterator<Item = Instruction>) -> (u32, u32) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        input.for_each(|instr| match instr {
            Instruction::Forward(amt) => x += amt,
            Instruction::Down(amt) => y += amt,
            Instruction::Up(amt) => y -= amt,
            Instruction::Empty => (),
        });

        (x, y)
    }

    pub fn part2_core(input: impl Iterator<Item = Instruction>) -> (u32, u32) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut aim: u32 = 0;
        input.for_each(|instr| match instr {
            Instruction::Down(amt) => aim += amt,
            Instruction::Up(amt) => aim -= amt,
            Instruction::Forward(amt) => {
                x += amt;
                y += aim * amt;
            }
            Instruction::Empty => (),
        });
        (x, y)
    }
}
