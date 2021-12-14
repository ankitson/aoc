use std::collections::{HashMap, HashSet};

// use crate::shared;
use crate::shared;
use itertools::Itertools;

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> usize {
        let lines = shared::parse(input);
        let mut count: usize = 0;
        for (_, second) in lines {
            let add = second
                .iter()
                .filter(|x| vec![2, 4, 3, 7].contains(&x.len()))
                .count();
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
        let lines = shared::parse(input);

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
