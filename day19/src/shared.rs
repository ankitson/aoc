use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize, pub isize);
impl Coord {
    pub fn l0_dist(&self) -> isize {
        self.0.abs().max(self.1.abs().max(self.2.abs()))
    }

    pub fn translate(&self, by: &Coord) -> Coord {
        let Coord(x, y, z) = self;
        let Coord(xo, yo, zo) = by;
        Coord(x + xo, y + yo, z + zo)
    }
}
pub type ScanCoords = HashMap<usize, Vec<Coord>>;

pub fn parse(input: &str) -> ScanCoords {
    let scanner_num_regex = Regex::new(r"--- scanner (\d+) ---").unwrap();
    let mut scan_coords: ScanCoords = HashMap::new();
    let mut coords = Vec::new();
    let mut current_scanner = None;
    for line in input.lines() {
        if line.is_empty() || line.chars().all(|c| c.is_ascii_whitespace()) {
            continue;
        }
        if let Some(matches) = scanner_num_regex.captures(line) {
            if current_scanner.is_some() {
                scan_coords.insert(current_scanner.unwrap(), coords.to_vec());
                coords.clear();
            }
            current_scanner = Some(matches.get(1).unwrap().as_str().parse().unwrap());
        } else if let Some((x, y, z)) = line.splitn(3, ',').map(|s| s.parse::<isize>().unwrap()).collect_tuple() {
            coords.push(Coord(x, y, z));
        }
    }
    if !coords.is_empty() {
        scan_coords.insert(current_scanner.unwrap(), coords.to_vec());
        coords.clear();
    }
    scan_coords
}

#[cfg(test)]
mod tests {
    use crate::shared::Coord;

    use super::parse;

    #[test]
    fn test_parse_sample() {
        let sample = include_str!("../inputs/sample.txt");
        let coord_map = parse(sample);
        assert_eq!(coord_map.len(), 5);
        assert_eq!(coord_map.get(&0).unwrap()[0], Coord(404, -588, -901));
    }

    #[test]
    fn test_parse_input() {
        let sample = include_str!("../inputs/day19.txt");
        let coord_map = parse(sample);
        assert_eq!(coord_map.len(), 35);
        assert_eq!(*coord_map.get(&34).unwrap().last().unwrap(), Coord(-660, 739, 490));
    }
}
