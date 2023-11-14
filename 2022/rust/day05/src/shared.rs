use itertools::Itertools;

pub fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize,usize,usize)>) {
    let lines = input.lines().collect_vec();
    let mut stacks: Vec<Vec<char>> = vec![vec![]];
    let mut moves = vec![];
    for line in &lines {
        if line.starts_with("move") {
            let parts = line.split(' ').collect_vec();
            let n1 = parts[1].parse::<usize>().expect("illegal");
            let n2 = parts[3].parse::<usize>().expect("illegal") - 1;
            let n3 = parts[5].parse::<usize>().expect("illegal") - 1;
            moves.push((n1,n2,n3));
        }
        else if line.is_empty() {
            continue;
        }
        else {
            let mut colidx = 0;
            for char in line.chars() {
                if char.is_alphabetic() {
                    let col = colidx / 4;
                    while stacks.len() < col + 1 {
                        stacks.push(vec![])
                    }
                    stacks[col].push(char);
                }   
                colidx += 1
            } 
        }
    }
    (stacks, moves)
}
