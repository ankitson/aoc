pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let parsed = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap() as u8))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    parsed
}
