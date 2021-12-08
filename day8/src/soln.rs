use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct Soln1 {}
pub struct BitVec {
    bits: u8,
}
impl BitVec {
    pub fn new(mut self) {
        self.bits = 0;
    }

    pub fn from(nums: Vec<usize>) -> Self {
        let bitvec = BitVec { bits: 0 };
        for num in nums {
            if (num > 7) {
                panic!("out of bounds")
            }
            let mask = (1 << num);
            bitvec.bits |= mask;
        }
        bitvec
    }

    pub fn set(mut self, index: u8) {
        if (index > 7) {
            panic!("illegal index")
        }
        let mask = (1 << index);
        self.bits |= mask;
    }

    pub fn get(self, index: u8) -> bool {
        if (index > 7) {
            panic!("illegal index")
        }
        let mask = (1 << index);
        ((self.bits & mask) >> index) & 1 == 1
    }

    pub fn len(self) -> u8 {
        let mut copy = self.bits.clone();
        let mut cnt = 0u8;
        while copy > 0 {
            if (copy & 1 == 1) {
                cnt += 1
            }
            copy = copy >> 1;
        }
        cnt
    }

    pub fn get_any(self) -> u8 {
        let mut i = 0;
        let mut mask = 1;
        while i < 8 {
            if (self.bits & mask == 1) {
                return i;
            }
            i += 1;
            mask <<= 1;
        }
        8
    }
}

impl Soln1 {
    fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();
        let lines = lines
            .iter()
            .map(|line| {
                let l = line.split('|').collect::<Vec<&str>>();
                let mut left = l[0].split_ascii_whitespace().collect::<Vec<&str>>();
                left.remove(left.len() - 1);
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
            println!("adding {} for line {:?}", add, second);
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
    * 7! mappings to start with
    * then filter
    *
    * mapping format
    * segment
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

        //TODO: Broken borrows
        let transform_lookup_letters = |letters: &str, perm: &[i32]| -> Option<&'static usize> {
            let letter_map = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

            let mut new_chars: Vec<char> = vec![];

            for letter in letters.chars() {
                let letter_idx = letter as u32 - 'a' as u32;
                let newpos = perm[letter_idx as usize];
                let newletter = letter_map[newpos as usize];
                new_chars.push(newletter);
            }
            new_chars.sort();
            let lookup: String = new_chars.into_iter().collect();
            digits.get(&*lookup)
        };

        //Return the letter this letter corresponds to under the permutation
        fn letter_map(letter: char, perm: Vec<i32>) -> char {
            let letter_map = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
            let letter_idx = letter as u32 - 'a' as u32;
            let newpos = perm[letter_idx as usize];
            let newletter = letter_map[newpos as usize];
            newletter
        }

        let perms = (0..6).permutations(7);
        let mut map: HashMap<String, usize> = HashMap::new();
        'outer: for (letters_to_digits, decode) in &lines {
            map = HashMap::new();
            for perm in perms {
                for word in letters_to_digits {
                    let mut wordc: Vec<char> = (*word).chars().collect();
                    wordc.sort();
                    let word: String = wordc.into_iter().collect();
                    //word = "cda"
                    //decrypt_word(perm, word) -> "acf"
                    //lookup_word("acf") -> 7
                    let lookup = transform_lookup_letters(&word, &perm);
                    match lookup {
                        Some(digit) => {
                            map.insert(word, *digit);
                            // map.insert();
                            ()
                        }
                        None => (),
                    }
                }
                if map.len() == 10 {
                    break 'outer;
                }
            }
        }
        5
    }
}
