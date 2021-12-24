use itertools::Itertools;

const A: u8 = 0;
const B: u8 = 1;
const C: u8 = 2;
const D: u8 = 3;
const EMPTY: u8 = 4;
const INVALID: u8 = 5;

pub fn parse<const N: usize>(input: &str) -> ([[u8; N]; 4]) {
    let mut lines = input.lines().skip(2);

    let row1 = lines.next().unwrap();
    let row2 = lines.next().unwrap();
    let mut cols: [[u8; N]; 4] = [[0u8; N]; 4];
    let mut col = [EMPTY; N];

    let mut col_idx = 0;
    for i in 0..row1.len() {
        let chtop = row1.chars().nth(i).unwrap();
        let chbot = row2.chars().nth(i).unwrap_or(' ');
        if (chtop == 'A' || chtop == 'B' || chtop == 'C' || chtop == 'D') {
            col[1] = ((chtop as u8) - ('A' as u8));
            col[2] = ((chbot as u8) - ('A' as u8));
            cols[col_idx] = col;
            col_idx += 1;
        }
    }
    cols
}

pub fn parse2(input: &str) -> ([[u8; 4]; 4]) {
    let mut lines = input.lines().skip(2);

    let row1 = lines.next().unwrap();
    let row2 = lines.next().unwrap();
    let mut cols = [[0; 4]; 4];
    let mut col = [0; 4];

    let rowm1 = vec!['D', 'C', 'B', 'A'];
    let rowm2 = vec!['D', 'B', 'A', 'C'];

    let mut col_idx = 0;
    for i in 0..row1.len() {
        let chtop = row1.chars().nth(i).unwrap();

        let chbot = row2.chars().nth(i).unwrap_or(' ');
        if (chtop == 'A' || chtop == 'B' || chtop == 'C' || chtop == 'D') {
            let chm1 = rowm1[col_idx]; //.unwrap_or(&' ');
            let chm2 = rowm2[col_idx]; //.get(i).unwrap_or(&' ');
            col[0] = ((chtop as u8) - ('A' as u8));
            col[1] = ((chm1 as u8) - ('A' as u8));
            col[2] = ((chm2 as u8) - ('A' as u8));
            col[3] = ((chbot as u8) - ('A' as u8));
            cols[col_idx] = col;
            col_idx += 1;
        }
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::{parse, parse2};

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        assert_eq!(parsed, [[1, 0], [2, 3], [1, 2], [3, 0]]);

        let parsed2 = parse2(sample);
        assert_eq!(parsed2, [[1, 3, 3, 0], [2, 2, 1, 3], [1, 1, 0, 2], [3, 0, 2, 0]]);
    }
}
