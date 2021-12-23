use itertools::Itertools;

pub fn parse(input: &str) -> ([[u8; 2]; 4]) {
    let mut lines = input.lines().skip(2);

    let row1 = lines.next().unwrap();
    let row2 = lines.next().unwrap();
    let mut cols = [[0; 2]; 4];
    let mut col = [0; 2];

    println!("row1: {:?}", &row1);
    println!("row2: {:?}", &row2);
    let mut col_idx = 0;
    for i in 0..row1.len() {
        let chtop = row1.chars().nth(i).unwrap();
        let chbot = row2.chars().nth(i).unwrap_or(' ');
        if (chtop == 'A' || chtop == 'B' || chtop == 'C' || chtop == 'D') {
            col[0] = ((chtop as u8) - ('A' as u8));
            col[1] = ((chbot as u8) - ('A' as u8));
            cols[col_idx] = col;
            col_idx += 1;
        }
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        assert_eq!(parsed, [[1, 0], [2, 3], [1, 2], [3, 0]])
    }
}
