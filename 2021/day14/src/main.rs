mod shared;
mod soln1;
mod soln2;

pub fn main() {
    println!("Hello Day 14!");
    let contents: &str = include_str!("../inputs/day14.txt");
    let part1_fast = soln2::part1(contents);
    println!("Part 1 (fast)=  {:?}", part1_fast);
    let part2_fast = soln2::part2(contents);
    println!("Part 2 (fast)=  {:?}", part2_fast);
}

#[cfg(test)]
mod tests {
    use day14::shared::parse2;

    use crate::shared::parse;
    use crate::soln1::Soln1;
    use crate::soln2;

    #[test]
    fn test_slow() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let parsed = parse(contents);
        println!("{:?}", parsed);
        let part1 = Soln1::part1(contents);
        println!("{}", part1);
        assert_eq!(part1, 1588);

        let mut soln = Soln1::new();
        let (poly, rules) = parse(contents);
        let part1_str = soln.expand_chunkwise(poly, &rules, 10);
        let part1 = Soln1::score(&part1_str);
        println!("{}", part1);
        assert_eq!(part1, 1588);

        // Too slow --
        // let part2 = soln.expand_chunkwise(poly, &rules, 40);
        // println!("{}", part1);
        // assert_eq!(part1, 1588);
        // let part2 = Soln1::part2(contents);
        // println!("{}", part2);
        // assert_eq!(part2, 2188189693529);
    }

    #[test]
    fn test_fast() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let part1_fast = soln2::part1(contents);
        println!("Part 1 (fast)=  {:?}", part1_fast);
        let part2_fast = soln2::part2(contents);
        println!("Part 2 (fast)=  {:?}", part2_fast);
    }

    #[test]
    fn test_iterate() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let (poly, rule_map) = parse2(contents);
        let mut map = soln2::build_map(poly);
        let new_map = soln2::iterate(&mut map, &rule_map);
        println!("{:?}", new_map);
    }

    #[test]
    fn test_expand() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let (poly, rules) = parse(contents);
        let mut soln = Soln1::new();
        let two_expand = soln.expand_n("NN", &rules, 2);
        println!("{}", two_expand);
        let th_expand = soln.expand_n("NN", &rules, 3);
        println!("{}", th_expand);
        println!("map after 1/2 expand NN:\n{:?}", soln.dp);
    }
}
