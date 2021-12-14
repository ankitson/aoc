use std::{fmt::Debug, iter};

pub struct Board {
    grid: Vec<Vec<usize>>,
}
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        for i in 0..self.grid.len() {
            str.push('\n');
            for j in 0..self.grid[i].len() {
                str.push_str(&self.grid[i][j].to_string());
                str.push(',');
            }
        }
        write!(f, "{}", str)
    }
}
impl Board {
    pub fn line(p1: (usize, usize), p2: (usize, usize)) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        if x1 == x2 {
            let max_y = usize::max(y1, y2);
            let min_y = usize::min(y1, y2);
            Box::new(iter::repeat(x1).take(max_y - min_y + 1).zip(min_y..max_y + 1))
        } else if y1 == y2 {
            let max_x = usize::max(x1, x2);
            let min_x = usize::min(x1, x2);
            Box::new(
                (min_x..max_x + 1)
                    .into_iter()
                    .zip(iter::repeat(y1).take(max_x - min_x + 1)),
            )
        } else {
            let max_x = usize::max(x1, x2);
            let min_x = usize::min(x1, x2);
            let max_y = usize::max(y1, y2);
            let min_y = usize::min(y1, y2);

            let x_range = min_x..max_x + 1;
            let y_range = min_y..max_y + 1;

            if x1 > x2 {
                let x_range = x_range.rev();
                if y1 > y2 {
                    let y_range = y_range.rev();
                    Box::new(x_range.zip(y_range))
                } else {
                    Box::new(x_range.zip(y_range))
                }
            } else {
                if y1 > y2 {
                    let y_range = y_range.rev();
                    Box::new(x_range.zip(y_range))
                } else {
                    Box::new(x_range.zip(y_range))
                }
            }
        }
    }
}

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str, grid_size: usize) -> usize {
        fn parse_coord_pair(raw_coords: &str) -> (usize, usize) {
            let r1 = raw_coords.split(',').collect::<Vec<&str>>();
            let x1 = r1[0].parse::<usize>().expect("illegal int");
            let y1 = r1[1].parse::<usize>().expect("illegal int");
            (x1, y1)
        }

        let mut board = Board {
            grid: vec![vec![0; grid_size]; grid_size],
        };
        input
            .lines()
            .map(|line| {
                let mut raw_coords = line.split(" -> ");
                let (x1, y1) = parse_coord_pair(raw_coords.next().unwrap());
                let (x2, y2) = parse_coord_pair(raw_coords.next().unwrap());
                ((x1, y1), (x2, y2))
            })
            .filter(|t| t.0 .0 == t.1 .0 || t.0 .1 == t.1 .1)
            .for_each(|((x1, y1), (x2, y2))| {
                for (x, y) in Board::line((x1, y1), (x2, y2)) {
                    board.grid[x][y] += 1
                }
            });
        // dbg!("{}", &board);
        board.grid.iter().flatten().filter(|x| **x > 1).count()
    }

    //identical to part1 but except the filter() on non axis aligned lines
    pub fn part2(input: &str, grid_size: usize) -> usize {
        fn parse_coord_pair(raw_coords: &str) -> (usize, usize) {
            let r1 = raw_coords.split(',').collect::<Vec<&str>>();
            let x1 = r1[0].parse::<usize>().expect("illegal int");
            let y1 = r1[1].parse::<usize>().expect("illegal int");
            (x1, y1)
        }

        let mut board = Board {
            grid: vec![vec![0; grid_size]; grid_size],
        };
        input
            .lines()
            .map(|line| {
                let mut raw_coords = line.split(" -> ");
                let (x1, y1) = parse_coord_pair(raw_coords.next().unwrap());
                let (x2, y2) = parse_coord_pair(raw_coords.next().unwrap());
                ((x1, y1), (x2, y2))
            })
            //.filter(|t| t.0 .0 == t.1 .0 || t.0 .1 == t.1 .1)
            .for_each(|((x1, y1), (x2, y2))| {
                for (x, y) in Board::line((x1, y1), (x2, y2)) {
                    board.grid[x][y] += 1
                }
            });
        // dbg!("{}", &board);
        board.grid.iter().flatten().filter(|x| **x > 1).count()
    }
}
