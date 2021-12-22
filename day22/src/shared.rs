use itertools::Itertools;

pub fn parse(input: &str) -> Vec<(u8, isize, isize, isize, isize, isize, isize)> {
    //on x=-20..26,y=-36..17,z=-47..7

    let mut cubes = Vec::new();
    let lines = input.lines();
    for line in lines {
        let (p, q) = line.split_once(' ').unwrap();
        let pb = {
            if p == "on" {
                1
            } else {
                0
            }
        };
        let r = q.split(',').collect_vec();
        let spl = (r[0].split("..").collect_vec());
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();
        println!("splh: {:?}", spl);
        let xl = splh.parse::<isize>().unwrap();
        let xh = spl[1].parse::<isize>().unwrap();
        let spl = r[1].split("..").collect_vec();
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();
        let yl = splh.parse::<isize>().unwrap();
        let yh = spl[1].parse::<isize>().unwrap();
        let spl = r[2].split("..").collect_vec();
        let splh: &str = &spl[0].chars().skip(2).collect::<String>();

        let zl = splh.parse::<isize>().unwrap();
        let zh = spl[1].parse::<isize>().unwrap();

        let row = (pb, xl, xh, yl, yh, zl, zh);
        cubes.push(row);
    }
    cubes
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        let sample = include_str!("../inputs/sample.txt");
        let parsed = parse(sample);
        assert_eq!(parsed.len(), 22);
        assert_eq!(parsed[1], (1, -20, 33, -21, 23, -26, 28));
    }
}
