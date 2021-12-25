use itertools::Itertools;

pub fn parse(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::parse;
    use itertools::Itertools;

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
    }
}
