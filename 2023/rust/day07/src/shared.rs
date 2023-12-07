use itertools::Itertools;

pub type Input = Vec<(Vec<char>, usize)>;
pub type Output = usize;

pub fn parse(raw_input: &'static str) -> Input {
    let mut pairs = raw_input
        .lines()
        .filter_map(|x| {
            if x.len() < 2 {
                return None;
            }
            let mut sp = x.split_ascii_whitespace();
            let hand = sp.next().unwrap().chars().collect_vec();
            let rank = sp.next().unwrap().parse::<usize>().unwrap();
            Some((hand, rank))
        })
        .collect_vec();
    pairs
}
