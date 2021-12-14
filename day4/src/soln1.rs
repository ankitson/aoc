use std::collections::HashSet;

const BOARD_SIZE: usize = 5;
pub struct Soln1 {}
impl Soln1 {
    pub fn parse(input: &str) -> (Vec<u8>, Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]>) {
        let mut lines = input.split('\n').filter(|x| !x.is_empty()).collect::<Vec<&str>>();

        let mut boards: Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

        let moves: Vec<u8> = lines[0]
            .split(',')
            .map(|x| x.parse::<u8>().expect("unable to parse num"))
            .collect::<Vec<u8>>();

        lines.remove(0);

        for board_lines in lines.chunks(BOARD_SIZE) {
            let mut board: [[i8; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                let line = board_lines[i];
                let nums: Vec<i8> = line
                    .split_whitespace()
                    .map(|x| x.parse::<i8>().unwrap_or_else(|x| panic!("unable to parse num {}", x)))
                    .clone()
                    .collect();
                board[i][..BOARD_SIZE].clone_from_slice(&nums[..BOARD_SIZE]);
            }
            boards.push(board);
        }

        (moves, boards)
    }

    pub fn unparse(output: (u32, u32)) -> String {
        (output.0 * output.1).to_string()
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
                        if board[ri][ci] == (num as i8) {
                            board[ri][ci] = -1;
                            board_row_filled[board_idx][ri] += 1;
                            if (board_row_filled[board_idx][ri] == BOARD_SIZE) {
                                winning_board_idx = Some(board_idx);
                                winning_num = Some(num);
                                break 'outer;
                            }
                            board_col_filled[board_idx][ci] += 1;
                            if (board_col_filled[board_idx][ci] == BOARD_SIZE) {
                                winning_board_idx = Some(board_idx);
                                winning_num = Some(num);
                                break 'outer;
                            }
                            //board_row_filled
                        }
                    }
                }
            }
        }

        let winning_board = boards[winning_board_idx.unwrap()];
        let sum = Self::board_sum(winning_board);
        (sum, winning_num.unwrap() as u32)
    }

    fn board_sum(board: [[i8; BOARD_SIZE]; BOARD_SIZE]) -> u32 {
        let x: i32 = board.iter().flatten().copied().filter(|x| *x > 0).map(i32::from).sum();
        x.try_into().unwrap()
    }

    pub fn part2(moves: Vec<u8>, mut boards: Vec<[[i8; BOARD_SIZE]; BOARD_SIZE]>) -> (u32, u32) {
        let mut board_row_filled: Vec<[usize; BOARD_SIZE]> = vec![[0; BOARD_SIZE]; boards.len()];
        let mut board_col_filled: Vec<[usize; BOARD_SIZE]> = vec![[0; BOARD_SIZE]; boards.len()];

        let mut winning_num: Option<u8> = None;
        let mut remaining_boards: HashSet<usize> = (0..boards.len()).into_iter().collect();
        let mut last_board: Option<usize> = None;
        'outer: for num in moves {
            for board_idx in 0..boards.len() {
                if !remaining_boards.contains(&board_idx) {
                    continue;
                }
                for ri in 0..BOARD_SIZE {
                    for ci in 0..BOARD_SIZE {
                        let board = &mut boards[board_idx];
                        if board[ri][ci] == (num as i8) {
                            board[ri][ci] = -1;
                            board_row_filled[board_idx][ri] += 1;
                            if board_row_filled[board_idx][ri] == BOARD_SIZE {
                                last_board = Some(board_idx);
                                remaining_boards.remove(&board_idx);
                                winning_num = Some(num);
                            }
                            board_col_filled[board_idx][ci] += 1;
                            if board_col_filled[board_idx][ci] == BOARD_SIZE {
                                last_board = Some(board_idx);
                                remaining_boards.remove(&board_idx);
                                winning_num = Some(num);
                            }
                        }
                    }
                }
            }
            if remaining_boards.is_empty() {
                break 'outer;
            }
        }

        let winning_board = boards[last_board.unwrap()];
        // println!(
        //     "Found a winning board: {:?} and number {}",
        //     winning_board,
        //     winning_num.unwrap()
        // );

        let sum = Self::board_sum(winning_board);
        (sum, winning_num.unwrap() as u32)
    }
}
