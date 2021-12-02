#[derive(Debug)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
    Empty,
}

fn instrs() -> impl Iterator<Item = Instruction> {
    let contents: &str = include_str!("../../input/day2.txt");
    contents.split('\n').map(|line| {
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

pub fn part1() -> String {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    instrs().for_each(|instr| match instr {
        Instruction::Forward(amt) => x += amt,
        Instruction::Down(amt) => y += amt,
        Instruction::Up(amt) => y -= amt,
        Instruction::Empty => (),
    });

    println!("Final Pos: {}, {}", x, y);
    (x * y).to_string()
}
