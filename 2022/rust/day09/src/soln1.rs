use std::collections::HashSet;
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let mut visited = HashSet::<(i32, i32)>::new();
        let mut coordh = (0, 0);
        let mut coordt = (0, 0);
        // let mut prev = (0, 0);
        visited.insert(coordt);
        input.lines().filter(|l| !l.is_empty()).for_each(|line| {
            let (dd, nn) = line.split_once(" ").expect("illegal line");
            let num = nn.parse::<i32>().expect("illegal num");
            let dir = dd.chars().nth(0).unwrap();
            // println!("line {} {}", dir, num);
            for _i in 0..num {
                coordh = Self::incr(dir, coordh);
                let xd = (coordh.0 as i32) - (coordt.0 as i32);
                let yd = (coordh.1 as i32) - (coordt.1 as i32);
                if xd.abs() > 1 || yd.abs() > 1 {
                    coordt.0 += xd.signum();
                    coordt.1 += yd.signum();
                }
                visited.insert(coordt);
                // prev = coord;
                // coord = Self::incr(dir, coord);

                // if i != num - 1 && num != 1 {
                // visited.insert(coord);
                // }
            }
            // println!("tail visited:"); // {:?}", visited);
            // Self::draw_visited(&visited);
        });
        // Self::draw_visited(&visited);
        visited.len()
        // Self::part1_core(shared::parse(input));
        // todo!()
    }

    #[allow(dead_code)]
    fn draw_visited(visited: &HashSet<(i32, i32)>) {
        let max_x = visited.iter().map(|(x, _y)| x).max().unwrap();
        let max_y = visited.iter().map(|(_x, y)| y).max().unwrap();
        // let mut grid = vec![];
        for y in (0..=*max_y).rev() {
            // let mut row = vec![];
            for x in 0..=*max_x {
                if visited.contains(&(x, y)) {
                    print!("X")
                } else {
                    print!(".")
                }
                // row.push(' ');
            }
            println!("")
            // grid.push(row);
        }

        // let x = vec![[0; max_x]; max_y];
    }

    fn incr(dir: char, coord: (i32, i32)) -> (i32, i32) {
        match dir {
            'U' => (coord.0, coord.1 + 1),
            'D' => (coord.0, coord.1 - 1),
            'L' => (coord.0 - 1, coord.1),
            'R' => (coord.0 + 1, coord.1),
            _ => panic!("ahh"),
        }
    }

    #[allow(dead_code)]
    pub fn part1_core(_grid: Vec<Vec<u32>>) -> i32 {
        todo!()
    }

    // pub fn part2(input: &str) -> i32 {
    //     // Self::part2_core(shared::parse(input));
    //     todo!();
    // }

    pub fn part2(input: &str) -> usize {
        let mut visited = HashSet::<(i32, i32)>::new();
        let mut positions = vec![(0, 0); 10];
        // let mut prev = (0, 0);
        visited.insert((0, 0));
        input.lines().filter(|l| !l.is_empty()).for_each(|line| {
            let (dd, nn) = line.split_once(" ").expect("illegal line");
            let num = nn.parse::<i32>().expect("illegal num");
            let dir = dd.chars().nth(0).unwrap();
            // println!("line {} {}", dir, num);
            for _i in 0..num {
                positions[0] = Self::incr(dir, positions[0]);
                for j in 1..10 {
                    let xd = positions[j - 1].0 - positions[j].0;
                    let yd = positions[j - 1].1 - positions[j].1;
                    if xd.abs() > 1 || yd.abs() > 1 {
                        positions[j].0 += xd.signum();
                        positions[j].1 += yd.signum();
                    }
                }
                visited.insert((positions[9].0, positions[9].1));
                // prev = coord;
                // coord = Self::incr(dir, coord);

                // if i != num - 1 && num != 1 {
                // visited.insert(coord);
                // }
            }
            // println!("tail visited:"); // {:?}", visited);
            // Self::draw_visited(&visited);
        });
        // Self::draw_visited(&visited);
        visited.len()
        // Self::part1_core(shared::parse(input));
        // todo!()
    }

    #[allow(dead_code)]
    pub fn part2_core(_grid: Vec<Vec<u32>>) -> i32 {
        todo!()
    }
}
