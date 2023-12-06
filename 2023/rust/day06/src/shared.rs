use itertools::Itertools;

pub type Input = (Vec<usize>, Vec<usize>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let (times, dists) = input
        .lines()
        .map(|l| l.split_ascii_whitespace().skip(1).map(|x| x.parse::<usize>().unwrap()).collect_vec())
        .collect_tuple()
        .unwrap();
    return (times, dists);
}
