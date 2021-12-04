use std::collections::HashMap;

const BOARD_SIZE: usize = 5;

const test_input: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

pub fn input() -> (Vec<u8>, Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]>) {
    //let contents = test_input;
    let contents: &str = include_str!("../../input/day4.txt");
    let mut lines = contents.split('\n').filter(|x| !x.is_empty()).collect::<Vec<&str>>();

    let mut boards: Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

    let moves: Vec<u8> = lines[0]
        .split(',')
        .map(|x| x.parse::<u8>().expect("unable to parse num"))
        .collect::<Vec<u8>>();

    lines.remove(0);

    for board_lines in lines.chunks(BOARD_SIZE) {
        println!("{:?}", board_lines);
        let mut board: [[i8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            let line = board_lines[i];
            let nums: Vec<i8> = line
                .split_whitespace()
                .map(|x| {
                    println!("x = {}", x);
                    x.parse::<i8>().expect(&format!("unable to parse num from {}", x))
                })
                .clone()
                .collect();
            board[i][..BOARD_SIZE].clone_from_slice(&nums[..BOARD_SIZE]);
        }
        boards.push(board);
    }

    (moves, boards)
}

pub fn part1(moves: Vec<u8>, mut boards: Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]>) -> (u32, u32) {
    let mut board_row_filled: Vec<[usize; BOARD_SIZE]> = vec![[0; BOARD_SIZE]; boards.len()]; //Vec::with_capacity(boards.len());
    let mut board_col_filled: Vec<[usize; BOARD_SIZE]> = vec![[0; BOARD_SIZE]; boards.len()];

    let mut winning_board_idx: Option<usize> = None;
    let mut winning_num: Option<u8> = None;
    'outer: for num in moves {
        for board_idx in 0..boards.len() {
            let mut board = &mut boards[board_idx];
            for ri in 0..BOARD_SIZE {
                for ci in 0..BOARD_SIZE {
                    if board[ri][ci] == num.try_into().expect("msg") {
                        board[ri][ci] = -1;
                        board_row_filled[board_idx][ri] += 1;
                        if (board_row_filled[board_idx][ri] == BOARD_SIZE) {
                            winning_board_idx = Some(board_idx);
                            winning_num = Some(num);
                            println!("{:?}", boards[winning_board_idx.unwrap()]);
                            break 'outer;
                        }
                        board_col_filled[board_idx][ci] += 1;
                        if (board_col_filled[board_idx][ci] == BOARD_SIZE) {
                            winning_board_idx = Some(board_idx);
                            winning_num = Some(num);
                            println!("{:?}", boards[winning_board_idx.unwrap()]);
                            break 'outer;
                        }
                        //board_row_filled
                    }
                }
            }
        }
    }

    let winning_board = boards[winning_board_idx.unwrap()];
    println!(
        "Found a winning board: {:?} and number {}",
        winning_board,
        winning_num.unwrap()
    );

    let sum = board_sum(winning_board);
    (sum, winning_num.unwrap() as u32)
}

fn board_sum(board: [[i8; BOARD_SIZE]; BOARD_SIZE]) -> u32 {
    let x: i32 = board
        .iter()
        .flatten()
        .copied()
        .filter(|x| *x > 0)
        .map(i32::from)
        //.map(|row| row.iter().map(|x| *x as u32).sum::<u32>())
        .sum();
    x.try_into().unwrap()
}
