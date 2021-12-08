use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct Soln1 {}
impl Soln1 {
    fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
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
    pub fn part1(input: &str) -> usize {
        let lines = Soln1::parse(input);
        let mut count: usize = 0;
        for (_, second) in lines {
            let add = second.iter().filter(|x| vec![2, 4, 3, 7].contains(&x.len())).count();
            count += add
        }
        count
    }

    /*
    lets number the segments
    *  0000
    * 1    2
    * 1    2
    *  3333
    * 4    5
    * 4    5
    *  6666
    *
    * candidate = mapping from letter to segment
    * (a-f) -> (0-6)
    *
    * 7! mappings to start with - one for each permutation of [0..6]
    */

    pub fn part2(input: &str) -> usize {
        let lines = Soln1::parse(input);

        let digits: HashMap<&str, usize> = HashMap::from_iter([
            ("abcefg", 0),
            ("cf", 1),
            ("acdeg", 2),
            ("acdfg", 3),
            ("bcdf", 4),
            ("abdfg", 5),
            ("abdefg", 6),
            ("acf", 7),
            ("abcdefg", 8),
            ("abcdfg", 9),
        ]);

        let transform_lookup_letters = |letters: &str, perm: &[i32]| -> Option<usize> {
            let letter_map = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

            let mut map: HashMap<char, char> = HashMap::new();
            for idx in 0..7 {
                let letter = (idx + b'a') as char;
                let mapletter = (perm[idx as usize] as u8 + b'a') as char;
                map.insert(letter, mapletter);
            }
            let mut new_chars: Vec<char> = vec![];

            for letter in letters.chars() {
                let letter_idx = letter as u32 - 'a' as u32;
                let newpos = perm[letter_idx as usize];
                let newletter = letter_map[newpos as usize];
                new_chars.push(newletter);
            }
            new_chars.sort();
            let lookup: String = new_chars.into_iter().collect();
            digits.get(&lookup as &str).cloned() //NOTE: digits.get(&lookup) fails because of type inference..
        };

        let mut decodedLines: Vec<usize> = vec![];
        let mut map: HashMap<String, usize> = HashMap::new();
        for (letters_to_digits, decode) in &lines {
            let mut maxHits = 0;
            let mut maxHitPerm: Vec<i32> = vec![0; 7];
            let mut maxHitMap: HashMap<String, usize> = HashMap::new();
            // println!("Decoding line: {:?} {:?}", &letters_to_digits, &decode);
            for perm in (0..7).permutations(7) {
                map = HashMap::new();
                for word in letters_to_digits {
                    let mut wordc: Vec<char> = (*word).chars().collect();
                    wordc.sort_unstable();
                    let word: String = wordc.into_iter().collect();
                    let lookup = transform_lookup_letters(&word, &perm);
                    if let Some(digit) = lookup {
                        map.insert(word, digit);
                    }
                }
                if map.len() == 10 {
                    maxHits = maxHits.max(map.len());
                    maxHitPerm = perm.clone();
                    maxHitMap = map.clone();
                    // dbg!("found working map: {:?}", map);
                    break;
                }
            }
            // dbg!("max hits = {} at perm {:?}", maxHits, maxHitPerm);
            let mut lineDecode = String::new();
            for word in decode {
                let mut wordc: Vec<char> = (*word).chars().collect();
                wordc.sort_unstable();
                let word: String = wordc.into_iter().collect();
                let digit = maxHitMap.get(&word).unwrap();
                lineDecode.push((*digit as u8 + '0' as u8) as char)
            }
            decodedLines.push(lineDecode.parse::<usize>().unwrap());
        }
        decodedLines.iter().sum()
    }
}

/*
 * Constraint propagation idea:
 * cgdf eagcbf fc adefg eacdb fbedga geafcd efc dacfe fdgaecb | dcefbag dgcf fc daefc
 *
 * 4 is the only number with 4 segs
 * we know cgdf = {bcdf} (in some order)
 *
 * representation of constraints:
 * c1 = map(
 *  c -> set(b,c,d,f),
 *  g -> set(b,c,d,f),
 *  d -> set(b,c,d,f),
 *  f -> set(b,c,d,f),
 *  a -> set(a,...,f),
 *  b -> set(a,...,f)
 * )
 *
 * now process eagcbf
 * eacgbf = 6 or 9
 * i.e eagcbf = {abdefg,abcdfg}
 * i.e eagcbf = {abcdefg}
 * this constraint map just allows any char to map to any char
 * call it cA, the open map
 * c2 = intersect(cA, c2) = c2
*/
pub struct Soln2 {
    // type ConstraintSet: HashSet<char>;
// type ConstraintMap: HashMap<char, ConstraintSet>;
}
impl Soln2 {
    // type ConstraintSet;
    // type ConstraintMap;
    // TODO: The constraints seem to be fine, but they dont converge.
    // Im not considering the entire constraint set together - only each character at a time
    pub fn part2(input: &str) -> usize {
        let lines = Soln1::parse(input);

        let chars: Vec<char> = ('a'..='g').into_iter().collect();
        let open_set: HashSet<char> = HashSet::from_iter(chars);
        let mut open_map: HashMap<char, HashSet<char>> = HashMap::new();
        for char in open_set.iter() {
            open_map.insert(*char, open_set.clone());
        }
        let open_map = open_map;

        for (words, to_decode) in lines {
            println!("Decoding line: {:?} {:?}", &words, &to_decode);
            let mut constraints = open_map.clone();
            for word in words {
                let new_constraints = Soln2::gen_constraints(word, &constraints);
                Soln2::merge_maps(&mut constraints, new_constraints);
                println!("constrains after {} =", &word);
                println!("{:?}", &constraints);
                if (Soln2::finished(&constraints)) {
                    dbg!(&word);
                    dbg!(&constraints);
                }
                // let new_constraints =
            }
            println!("final constraints:");
            println!("{:?}", &constraints);
        }
        5
    }

    fn finished(constraints: &HashMap<char, HashSet<char>>) -> bool {
        for (k, v) in constraints.iter() {
            if (v.len() != 1) {
                return false;
            }
        }
        true
    }

    fn gen_constraints(
        word: &str,
        existing_constraints: &HashMap<char, HashSet<char>>,
    ) -> HashMap<char, HashSet<char>> {
        let digits: HashMap<usize, &str> = HashMap::from_iter([
            (0, "abcefg"),
            (1, "cf"),
            (2, "acdeg"),
            (3, "acdfg"),
            (4, "bcdf"),
            (5, "abdfg"),
            (6, "abdefg"),
            (7, "acf"),
            (8, "abcdefg"),
            (9, "bcdfg"),
        ]);
        let candidates: Vec<&str> = digits
            .iter()
            .filter(|(k, v)| v.len() == word.len())
            .map(|x| *x.1)
            .into_iter()
            .collect();
        for digit in 0..10 {
            let digit_segments = digits.get(&digit).unwrap();
        }
        let mut constraints: HashMap<char, HashSet<char>> = HashMap::new();
        for char in word.chars() {
            let mut could_map_to: HashSet<char> = HashSet::new();
            for candidate in &candidates {
                for candidate_char in candidate.chars() {
                    if (existing_constraints.get(&char).unwrap().contains(&candidate_char)) {
                        could_map_to.insert(candidate_char);
                    }
                }
            }
            constraints.insert(char, could_map_to);
        }
        constraints
    }

    pub fn merge_maps(m1: &mut HashMap<char, HashSet<char>>, m2: HashMap<char, HashSet<char>>) {
        let existing_keys: Vec<char> = m1.keys().into_iter().cloned().collect();
        for char in &existing_keys {
            if (m2.contains_key(char)) {
                let merge: HashSet<char> = m1
                    .get(char)
                    .unwrap()
                    .intersection(&m2.get(char).cloned().unwrap())
                    .into_iter()
                    .cloned()
                    .collect();
                m1.insert(*char, merge);
            }
        }
        for char in m2.keys() {
            if (!existing_keys.contains(&char)) {
                m1.insert(*char, m2.get(char).cloned().unwrap());
            } else {
            }
        }
    }
}
