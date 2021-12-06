mod soln;

pub fn main() {
    let contents: &str = include_str!("../day5.txt");
    let parsed = soln::Soln1::part1(contents, 1000);
    println!("{:?}", parsed);
}

#[cfg(test)]
mod tests {
    use crate::soln;

    const INPUT1: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let soln = soln::Soln1::part1(INPUT1, 10);
        assert_eq!(soln, 5);
    }
}
