/* 
 Taken from alion02 on #advent-of-code-2024 in Rust Discord: https://discord.com/channels/273534239310479360/386246790565068811/1317361239931883552 
 It compresses each grid and prints the minimum - the most patterned grid should have the lowest entropy and hence the lowest compressed size
*/

let input = read_to_string("../inputs/14.txt").unwrap();
// let map: Vec<_> = input.lines().map(|l| l.as_bytes().trim_ascii()).collect();

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

let mut robots: Vec<_> = Regex::new("(\\d|-)+")
    .unwrap()
    .find_iter(&input)
    .chunks(4)
    .into_iter()
    .map(|mut i| Robot {
        px: i.next().unwrap().as_str().parse().unwrap(),
        py: i.next().unwrap().as_str().parse().unwrap(),
        vx: i.next().unwrap().as_str().parse().unwrap(),
        vy: i.next().unwrap().as_str().parse().unwrap(),
    })
    .collect();

println!("{robots:?}");

let mut min = usize::MAX;
for i in 0.. {
    let mut grid = [b'.'; 101 * 103];
    for robot in &mut robots {
        grid[robot.py as usize * 101 + robot.px as usize] = b'O';
        robot.px = ((robot.px + robot.vx) % 101 + 101) % 101;
        robot.py = ((robot.py + robot.vy) % 103 + 103) % 103;
    }
    let mut vec = vec![];
    let mut encoder = DeflateEncoder::new(&mut vec, Compression::best());
    encoder.write_all(&grid).unwrap();
    encoder.finish().unwrap();
    if vec.len() < min {
        min = vec.len();
        println!("{i}:");
        for y in 0..103 {
            println!("{}", str::from_utf8(&grid[y * 101..y * 101 + 101]).unwrap());
        }
    }
}
