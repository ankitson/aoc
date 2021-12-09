pub fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let lines = lines
        .iter()
        .map(|line| {
            let l = line.split('|').collect::<Vec<&str>>();
            let left = l[0].split_ascii_whitespace().collect::<Vec<&str>>();
            let right = l[1].split_ascii_whitespace().collect::<Vec<&str>>();
            (left, right)
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>();
    lines
}
