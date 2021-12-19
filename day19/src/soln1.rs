use crate::shared::parse;

///
/// 1D case:
/// scanner at 0, beacon at 1
/// possible reported positions: [1,-1]
///
/// 2D case:
/// scanner at (0,0), beacon at (1,2)
/// rotate around Z axis
/// 0   : x->x, y->y
/// 90  : x->y, y->-x
/// 180 : x->-x, y->-y
/// 270 : x->y, y->x
/// map positive x to one of 4 directions
/// then map positive y to one of 2 directions? (but 3rd is ruled out because it will be = -x)
/// so 4*2 = 8 total orientations?
///
/// rotation matrix:
/// [1 0] * [1 0]  = [1 0]
///         [0 1]
///
/// 3D case:
///
/// face +- X,Y or Z -> 12 choices
/// consider
///
///
/// rotate
/// ]
pub fn part1(input: &str) -> usize {
    todo!()
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::soln1::{part1, part2};

    #[test]
    fn test_part1() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part1 = part1(sample);
        println!("Part 1 (sample1) = {:?}", part1);
        // assert_eq!(part1, 0);
    }

    #[test]
    fn test_part2() {
        let sample: &str = include_str!("../inputs/sample.txt");
        let part2 = part2(sample);
        println!("Part 2 (sample2) = {:?}", part2);
        // assert_eq!(part2, 0);
    }
}
