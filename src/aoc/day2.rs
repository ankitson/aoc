pub fn part1() -> String {
    let contents: &str = include_str!("../../input/day2.txt");

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_ascii_whitespace();
        let dir = parts.next().expect("illegal line");
        let amount = parts
            .next()
            .expect("illegal line")
            .parse::<i32>()
            .expect("illegal line");

        match dir {
            "forward" => x += amount,
            "down" => y += amount,
            "up" => y -= amount,
            _ => panic!("illegal amount"),
        }
    }
    println!("Final Pos: {}, {}", x, y);
    (x * y).to_string()
}
