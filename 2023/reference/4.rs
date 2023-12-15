//from rust discord, magnus hegdahl:
//https://discord.com/channels/273534239310479360/386246790565068811/1181170969671372841
//fast soln to aoc 2023 day 04
use aoc_zen_runner_macros::aoc;

#[aoc(2023, day4)]
pub mod solutions {
    use aoc_zen_runner_macros::solution;

    // const USELESS_PREFIX_LEN: usize = "Card x: ".len();
    const USELESS_PREFIX_LEN: usize = "Card xxx: ".len();

    #[solution(part1, bad_but_fast)]
    pub fn part1(input: &str) -> u32 {
        unsafe {
            let input = input.as_bytes();

            let mut char_i = 0;

            let mut answer = 0u32;
            while char_i != input.len() {
                char_i += USELESS_PREFIX_LEN;
                let mut have = [0u8; 26 * 10];
                while *input.get_unchecked(char_i) != b'|' {
                    let c0 = (*input.get_unchecked(char_i) - b' ') as usize;
                    let c1 = (*input.get_unchecked(char_i + 1) - b'0') as usize;
                    *have.get_unchecked_mut(c0 * 10 + c1) = 1;
                    char_i += 3;
                }
                char_i += 2;

                let mut matches = 0;

                while *input.get_unchecked(char_i - 1) != b'\n' {
                    let c0 = (*input.get_unchecked(char_i) - b' ') as usize;
                    let c1 = (*input.get_unchecked(char_i + 1) - b'0') as usize;
                    matches += *have.get_unchecked(c0 * 10 + c1) as usize;
                    char_i += 3;
                }

                if matches != 0 {
                    answer += 1 << (matches - 1);
                }
            }

            answer
        }
    }

    #[solution(part2, bad_but_fast)]
    pub fn part2(input: &str) -> u64 {
        unsafe {
            let input = input.as_bytes();
            let mut current = 1u64;
            let mut changes = [0u64; 210];

            let mut line_i = 0;
            let mut char_i = 0;

            let mut answer = 0u64;
            while char_i != input.len() {
                current = current.wrapping_add(*changes.get_unchecked(line_i));
                answer += current;

                char_i += USELESS_PREFIX_LEN;
                let mut have = [0u8; 26 * 10];
                while *input.get_unchecked(char_i) != b'|' {
                    let c0 = (*input.get_unchecked(char_i) - b' ') as usize;
                    let c1 = (*input.get_unchecked(char_i + 1) - b'0') as usize;
                    *have.get_unchecked_mut(c0 * 10 + c1) = 1;
                    char_i += 3;
                }
                char_i += 2;

                let mut matches = 0;

                while *input.get_unchecked(char_i - 1) != b'\n' {
                    let c0 = (*input.get_unchecked(char_i) - b' ') as usize;
                    let c1 = (*input.get_unchecked(char_i + 1) - b'0') as usize;
                    matches += *have.get_unchecked(c0 * 10 + c1) as usize;
                    char_i += 3;
                }

                line_i += 1;
                *changes.get_unchecked_mut(line_i) =
                    changes.get_unchecked(line_i).wrapping_add(current);
                *changes.get_unchecked_mut(line_i + matches) = changes
                    .get_unchecked(line_i + matches)
                    .wrapping_sub(current);
            }

            answer
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(13, 30)]
    const INPUT1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
}
