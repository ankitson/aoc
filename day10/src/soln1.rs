#[path = "shared.rs"]
mod shared;

enum Either<A, B> {
    Left(A),
    Right(B),
}

/**
 * Using match lookups
 */
pub struct Soln1 {}
impl Soln1 {
    #[inline]
    fn score(c: &char) -> Option<i32> {
        match *c {
            '(' => Some(1),
            '[' => Some(2),
            '{' => Some(3),
            '<' => Some(4),
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None,
        }
    }

    #[inline]
    fn open_idx(c: &char) -> Option<i32> {
        match *c {
            '(' => Some(0),
            '[' => Some(1),
            '{' => Some(2),
            '<' => Some(3),
            _ => None,
        }
    }

    #[inline]
    fn idx_open(i: i32) -> Option<char> {
        match i {
            0 => Some('('),
            1 => Some('['),
            2 => Some('{'),
            3 => Some('<'),
            _ => None,
        }
    }

    #[inline]
    fn close_idx(i: char) -> Option<i32> {
        match i {
            ')' => Some(0),
            ']' => Some(1),
            '}' => Some(2),
            '>' => Some(3),
            _ => None,
        }
    }

    fn first_illegal(line: &str) -> Either<usize, Vec<char>> {
        let mut stack: Vec<char> = Vec::new();
        let mut illegal_idx: Option<usize> = None;
        for (idx, char) in line.chars().enumerate() {
            if let Some(_) = Self::open_idx(&char) {
                stack.push(char)
            } else {
                let br_type = Self::close_idx(char).expect("expected close");
                let matching_open =
                    Self::idx_open(br_type).expect("no matching open");
                if *stack.last().unwrap() == matching_open {
                    stack.pop();
                } else {
                    illegal_idx = Some(idx);
                    break;
                }
            }
        }
        if let Some(bad_idx) = illegal_idx {
            Either::Left(bad_idx)
        } else {
            Either::Right(stack.clone())
        }
    }

    pub fn part1(input: &str) -> i32 {
        let lines = shared::parse(input);
        let mut total_score = 0;
        for line in lines {
            let line_score = match Self::first_illegal(line) {
                Either::Left(bad_index) => {
                    let bad_char =
                        line.chars().nth(bad_index).expect("illegal char");
                    Self::score(&bad_char).expect("no score for char")
                }
                _ => 0,
            };
            total_score += line_score;
        }
        total_score
    }

    fn score_completion(completion: Vec<char>) -> i64 {
        let mut score = 0i64;
        for char in completion.iter().rev() {
            let br_score = Self::score(&char).expect("no score for char");
            score = (score * 5) + (i64::from(br_score));
        }
        score
    }

    pub fn part2(input: &str) -> i64 {
        let lines = shared::parse(input);
        let mut scores: Vec<i64> = vec![];
        for line in lines {
            let case = Self::first_illegal(line);
            if let Either::Right(stack) = case {
                let completion_score = Self::score_completion(stack);
                scores.push(completion_score);
            }
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
        // let pos = _scores.len() / 2;
        // _scores.select_nth_unstable(pos);
        // _scores[pos]
    }
}
