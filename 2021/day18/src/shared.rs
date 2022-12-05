use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<(usize, usize)>> {
    input.lines().map(parse_one).collect_vec()
}

/// [[6,[5,[4,[3,2]]]], 1]
/// becomes (num,depth) pairs
/// [(6,1),(5,2),(4,3),(3,4),(2,4),(1,0)]
pub fn parse_one(input: &str) -> Vec<(usize, usize)> {
    let chars = input.chars().collect_vec();
    let mut vec = Vec::new();
    parse_rec(&chars, 0, &mut vec, "root".to_string());
    vec
}

fn parse_comma(chars: &[char], depth: usize, label: String) -> usize {
    // println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || chars[0] != ',' {
        panic!()
    }
    1
}

fn parse_open(chars: &[char], depth: usize, label: String) -> usize {
    // println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || chars[0] != '[' {
        panic!()
    }
    1
}

fn parse_close(chars: &[char], depth: usize, label: String) -> usize {
    // println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || chars[0] != ']' {
        panic!()
    }
    1
}

fn parse_num(chars: &[char], depth: usize, label: String) -> (usize, usize) {
    // println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() || !chars[0].is_numeric() {
        panic!()
    }
    let mut i = 0;
    while i < chars.len() && chars[i].is_numeric() {
        i += 1;
    }
    let tempstr: String = chars[0..i].iter().collect();
    let numval = tempstr.parse::<usize>().unwrap();
    //(usize::from_str_radix((chars[0..i].iter().collect()), 10).unwrap(), i)
    //(chars[0].to_digit(10).unwrap().try_into().unwrap(), 1)
    (numval, i)
}

/// EXPR = [EXPR,EXPR]
/// EXPR = LIT
pub fn parse_rec(chars: &[char], depth: usize, vec: &mut Vec<(usize, usize)>, label: String) -> usize {
    // println!("{}parse[{}]: {:?}", ".".repeat(depth), label, chars);
    if chars.is_empty() {
        return 0;
    }

    if chars[0].is_numeric() {
        let (n, np) = parse_num(&chars, depth, "lit".to_string());
        vec.push((n, depth));
        np
    } else {
        let mut np = 0;
        np += parse_open(chars, depth, "open".to_string());
        let npleft = parse_rec(&chars[np..], depth + 1, vec, "left".to_string());
        np += npleft;
        np += parse_comma(&chars[np..], depth + 1, "comma".to_string());
        let npright = parse_rec(&chars[np..], depth + 1, vec, "right".to_string());
        np += npright;
        np += parse_close(&chars[np..], depth, "close".to_string());

        np
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, parse_one, parse_rec};

    fn testcases_one() -> Vec<(&'static str, Vec<(usize, usize)>)> {
        vec![
            ("6", vec![(6, 0)]),
            ("[1,2]", vec![(1, 1), (2, 1)]),
            ("[1,[2,[3,[4,4]]]]", vec![(1, 1), (2, 2), (3, 3), (4, 4), (4, 4)]),
            ("[1,[2,[31,[4,14]]]]", vec![(1, 1), (2, 2), (31, 3), (4, 4), (14, 4)]),
        ]
    }

    fn testcases_many() -> Vec<(&'static str, Vec<Vec<(usize, usize)>>)> {
        vec![(
            include_str!("../inputs/sample3.txt"),
            vec![
                vec![(1, 1), (1, 1)],
                vec![(2, 1), (2, 1)],
                vec![(3, 1), (3, 1)],
                vec![(4, 1), (4, 1)],
                vec![(5, 1), (5, 1)],
                vec![(6, 1), (6, 1)],
            ],
        )]
    }

    #[test]
    fn test_parse_one() {
        for (str, expected_parse) in &testcases_one() {
            let vec = parse_one(str);
            assert_eq!(vec, *expected_parse);
        }
    }

    #[test]
    fn test_parse_many() {
        for (str, expected_parse) in &testcases_many() {
            let vec = parse(str);
            assert_eq!(vec, *expected_parse);
        }
    }
}
