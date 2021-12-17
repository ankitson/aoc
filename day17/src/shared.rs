use itertools::Itertools;

pub fn parse(input: &str) -> ((isize, isize), (isize, isize)) {
    let c1 = input.find(':').unwrap();
    let i1 = input[c1 + 1..].trim();
    let (mut xr, mut yr) = i1.split_once(',').unwrap();
    xr = xr.trim();
    yr = yr.trim();
    let (xl, xh) = xr[2..].split_once("..").unwrap();
    let (yl, yh) = yr[2..].split_once("..").unwrap();
    let coords = vec![xl, xh, yl, yh]
        .iter()
        .map(|n| n.parse::<isize>().unwrap())
        .collect_vec();
    assert!(coords.len() == 4);
    ((coords[0], coords[1]), (coords[2], coords[3]))
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn test_parse() {
        let sample: &str = include_str!("../inputs/sample.txt");
        assert_eq!(parse(sample), ((20, 30), (-10, -5)));

        let input: &str = include_str!("../inputs/day17.txt");
        assert_eq!(parse(input), ((137, 171), (-98, -73)));
    }
}
