#![feature(generic_const_exprs)]

use rayon::prelude::*;

struct Grid<T, const HEIGHT: usize, const WIDTH: usize> {
    data: [[T; WIDTH]; HEIGHT],
}

impl<T, const HEIGHT: usize, const WIDTH: usize> std::ops::Index<(usize, usize)>
    for Grid<T, HEIGHT, WIDTH>
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe { self.data.get_unchecked(index.0).get_unchecked(index.1) }
    }
}

impl<T, const HEIGHT: usize, const WIDTH: usize> std::ops::IndexMut<(usize, usize)>
    for Grid<T, HEIGHT, WIDTH>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe {
            self.data
                .get_unchecked_mut(index.0)
                .get_unchecked_mut(index.1)
        }
    }
}

#[inline(always)]
unsafe fn solve<const MAX_S_LEN: usize, const MAX_C_LEN: usize>(s: &[u8], c: &[u8]) -> u64
where
    [(); MAX_S_LEN + 1]:,
    [(); MAX_C_LEN + 1]:,
{
    let n = s.len() - 1;
    let m = c.len();

    let mut next_forced_space = [n; MAX_S_LEN + 1];
    for (i, c) in s[..n].iter().copied().enumerate().rev() {
        if c == b'.' {
            *next_forced_space.get_unchecked_mut(i) = i;
        } else {
            *next_forced_space.get_unchecked_mut(i) = *next_forced_space.get_unchecked(i + 1);
        }
    }

    let mut ways = Grid {
        data: [[0; MAX_C_LEN + 1]; MAX_S_LEN + 1],
    };
    ways[(0, 0)] = 1;

    for (i, chr) in s[..n].iter().copied().enumerate() {
        for (j, w) in c.iter().copied().enumerate() {
            let w = w as usize;

            // can we use i as gap?
            if chr != b'#' {
                ways[(i + 1, j)] += ways[(i, j)];
            }

            // can we build w starting at i?
            if *next_forced_space.get_unchecked(i) >= i + w
                && i + w <= n
                && *s.get_unchecked(i + w) != b'#'
            {
                ways[(n.min(i + w + 1), j + 1)] += ways[(i, j)];
            }
        }

        if chr != b'#' {
            ways[(i + 1, m)] += ways[(i, m)];
        }
    }

    ways[(n, m)]
}

pub fn part1(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| unsafe {
            let mut count_store = [0u8; 6];
            let mut input = line.as_bytes();
            let space = memchr::memchr(b' ', input).unwrap_unchecked();
            let s = input.get_unchecked(..space + 1);
            input = input.get_unchecked(space + 1..);

            count_store[0] = 0;
            let mut count_count = 0;

            while *input.get_unchecked(0) != b'\n' {
                if *input.get_unchecked(0) == b',' {
                    count_count += 1;
                    *count_store.get_unchecked_mut(count_count) = 0;
                } else {
                    *count_store.get_unchecked_mut(count_count) *= 10;
                    *count_store.get_unchecked_mut(count_count) += input.get_unchecked(0) - b'0';
                }
                input = input.get_unchecked(1..);
            }
            count_count += 1;

            solve::<20, 6>(s, &count_store.get_unchecked(..count_count))
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| unsafe {
            let mut s_store = [0u8; 21 * 5 + 5];
            let mut count_store = [0u8; 6 * 5];
            let mut input = line.as_bytes();
            let space = memchr::memchr(b' ', input).unwrap_unchecked();
            let s = input.get_unchecked(..space);
            input = input.get_unchecked(space + 1..);

            s_store[..s.len()].copy_from_slice(s);
            s_store[s.len()] = b'?';
            for i in 1..5 {
                s_store.copy_within(0..s.len() + 1, (s.len() + 1) * i);
            }

            count_store[0] = 0;
            let mut count_count = 0;

            while *input.get_unchecked(0) != b'\n' {
                if *input.get_unchecked(0) == b',' {
                    count_count += 1;
                    *count_store.get_unchecked_mut(count_count) = 0;
                } else {
                    *count_store.get_unchecked_mut(count_count) *= 10;
                    *count_store.get_unchecked_mut(count_count) += input.get_unchecked(0) - b'0';
                }
                input = input.get_unchecked(1..);
            }
            count_count += 1;

            for i in 1..5 {
                count_store.copy_within(0..count_count, count_count * i);
            }

            solve::<{ 20 * 5 + 4 }, { 6 * 5 }>(
                &s_store.get_unchecked(..s.len() * 5 + 5),
                &count_store.get_unchecked(..count_count * 5),
            )
        })
        .sum()
}

pub fn run(input: &str) -> impl std::fmt::Display {
    part1(input)
}

use aoc_zen_runner_macros::aoc;

#[aoc(2023, day12)]
pub mod solutions {
    use aoc_zen_runner_macros::solution;

    #[solution(part1, iterative)]
    pub fn part1(input: &str) -> u64 {
        super::part1(input)
    }
    #[solution(part2, iterative)]
    pub fn part2(input: &str) -> u64 {
        super::part2(input)
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;

    #[allow(non_upper_case_globals)]
    #[aoc_case(21, 525152)]
    const input1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
}
