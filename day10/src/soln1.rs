use lazy_static::lazy_static;
use phf::phf_map;

#[path = "shared.rs"]
mod shared;

enum Either<A, B> {
    Left(A),
    Right(B),
}

lazy_static! {
    static ref OPEN_TO_CLOSE: phf::Map<char, char> = phf_map! {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>'
    };
    static ref CLOSE_TO_OPEN: phf::Map<char, char> = phf_map! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<'
    };
    static ref SCORES: phf::Map<char, i32> = {
        // println!("initializing SCORES");
        phf_map! {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4
        }
    };
}

/**
 * Using lazy_statics to avoid
 */
pub struct Soln1 {}
impl Soln1 {
    pub fn init_statics() {
        #[allow(dead_code)]
        OPEN_TO_CLOSE.get(&'(');
        #[allow(dead_code)]
        CLOSE_TO_OPEN.get(&')');
        #[allow(dead_code)]
        SCORES.get(&'(');
    }

    fn first_illegal(line: &str) -> Either<usize, Vec<char>> {
        let mut stack: Vec<char> = Vec::new();
        let mut illegal_idx: Option<usize> = None;
        for (idx, char) in line.chars().enumerate() {
            if OPEN_TO_CLOSE.contains_key(&char) {
                stack.push(char);
            } else {
                let matching_open =
                    CLOSE_TO_OPEN.get(&char).expect("illegal char");
                if *stack.last().unwrap() == *matching_open {
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

    fn score_illegal(bracket: &char) -> i32 {
        // println!("running score_illegal");
        *SCORES.get(bracket).expect("illegal bracket")
    }

    pub fn part1(input: &str) -> i32 {
        // println!("running part1");
        let lines = shared::parse(input);
        let mut total_score = 0;
        for line in lines {
            let line_score = match Self::first_illegal(line) {
                Either::Left(bad_index) => {
                    let bad_char =
                        line.chars().nth(bad_index).expect("illegal char");
                    let score = Self::score_illegal(&bad_char);
                    score
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
            let br_score = *SCORES.get(char).expect("illegal char");
            score = (score * 5) + (i64::from(br_score));
        }
        score
    }

    pub fn part2(input: &str) -> i64 {
        // println!("running part2");
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
