#[path = "shared.rs"]
mod shared;

enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct Soln1 {}
impl Soln1 {
    fn first_illegal(line: &str) -> Either<usize, Vec<char>> {
        let br_opens = vec!['(', '[', '{', '<'];
        let br_closes = vec![')', ']', '}', '>'];
        let mut stack: Vec<char> = Vec::new();
        let mut illegal_idx: Option<usize> = None;
        for (idx, char) in line.chars().enumerate() {
            if br_opens.iter().any(|c| *c == char) {
                stack.push(char);
            } else {
                let closing_bracket_id = br_closes
                    .iter()
                    .position(|c| *c == char)
                    .unwrap_or_else(|| panic!("unexpected char {}", char));
                let matching_open = br_opens[closing_bracket_id];
                if stack[stack.len() - 1] == matching_open {
                    stack.remove(stack.len() - 1);
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

    fn score_illegal(bracket: &char) -> usize {
        match bracket {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("illegal char"),
        }
    }

    pub fn part1(input: &str) -> usize {
        let lines = shared::parse(input);
        let mut total_score = 0;
        for line in lines {
            let line_score = match Self::first_illegal(line) {
                Either::Left(bad_index) => Self::score_illegal(&line.chars().nth(bad_index).unwrap()),
                _ => 0,
            };
            total_score += line_score;
        }
        total_score
    }

    fn score_completion(completion: Vec<char>) -> usize {
        let mut score = 0;
        let br_opens = vec!['(', '[', '{', '<'];
        let char_scores = vec![1, 2, 3, 4];
        for char in completion.iter().rev() {
            let bracket_id = br_opens
                .iter()
                .position(|c| c == char)
                .unwrap_or_else(|| panic!("unexpected char {}", char));
            score = (score * 5) + char_scores[bracket_id];
        }
        score
    }

    pub fn part2(input: &str) -> usize {
        let lines = shared::parse(input);
        let mut scores: Vec<usize> = vec![];
        for line in lines {
            let case = Self::first_illegal(line);
            if let Either::Right(stack) = case {
                let completion_score = Self::score_completion(stack);
                scores.push(completion_score);
            }
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}
