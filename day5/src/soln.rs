use shared;
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
    //todo: this wont compile
    pub fn axis_line(p1: (usize, usize), p2: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        if x1 == x2 {
            let maxY = usize::max(y1, y2);
            let minY = usize::min(y1, y2);
            iter::repeat(x1).take(maxY - minY + 1).zip(minY..maxY + 1)
        } else if y1 == y2 {
            let maxX = usize::max(x1, x2);
            let minX = usize::min(x1, x2);
            (minX..maxX + 1).into_iter().zip(iter::repeat(y1).take(maxX - minX + 1))
        } else {
            panic!("not an axis line")
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
                for (x, y) in Board::axis_line((x1, y1), (x2, y2)) {
                    println!("drawing point {},{} for line ({},{}) - ({},{})", x, y, x1, y1, x2, y2);
                    board.grid[x][y] += 1
                }
            });
        dbg!("{}", &board);
        board.grid.iter().flatten().filter(|x| **x > 1).count()
    }
}
