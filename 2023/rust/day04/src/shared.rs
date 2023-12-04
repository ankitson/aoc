use itertools::Itertools;

pub type Input = Vec<(Vec<usize>, Vec<usize>)>;
pub type Output = usize;

//Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (_, suff) = line.split_once(':').unwrap();
            let (wins, haves) = suff.split_once('|').unwrap();
            let wins = wins.split_ascii_whitespace().filter_map(|x| {
                if x.len() > 0 {
                    Some(x.parse::<usize>().unwrap())
                } else {
                    None
                }
            });
            let haves = haves.split_ascii_whitespace().filter_map(|x| {
                if x.len() > 0 {
                    Some(x.parse::<usize>().unwrap())
                } else {
                    None
                }
            });
            (wins.collect_vec(), haves.collect_vec())
        })
        .collect_vec()
}
