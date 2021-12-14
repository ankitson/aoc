use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    iter::Map,
};

use itertools::PeekingNext;

use crate::shared::{parse, Rule};

pub struct Soln1 {
    // pub dp: [[[String; 10]; 26]; 26],
    pub dp: BTreeMap<usize, HashMap<String, String>>,
}

impl Soln1 {
    pub fn char_idx(char: char) -> usize {
        (char.to_ascii_lowercase() as u8 - 'a' as u8).into()
    }

    pub fn new() -> Self {
        let dp = BTreeMap::new();
        // let dp = [[["".to_string(); 10]; 26]; 26];
        Soln1 { dp }
    }

    pub fn apply_n(poly: &str, rules: Vec<Rule>, n: usize) -> String {
        let mut final_poly: String = poly.to_string();
        for i in 0..n {
            final_poly = Self::apply(&final_poly, &rules);
        }
        final_poly
    }

    pub fn score(poly: &str) -> usize {
        let mut max_freq: usize = 0;
        let mut min_freq: usize = usize::MAX;
        let mut max_freq_char: char = 'a';
        let mut min_freq_char: char = 'a';

        let mut freqs: HashMap<char, usize> = HashMap::new();
        for char in poly.chars() {
            let mut newf = *freqs.get(&char).unwrap_or((&0));
            newf += 1;
            freqs.insert(char, newf);
            let new_freq = *freqs.get(&char).unwrap();
            if (new_freq >= max_freq || char == max_freq_char) {
                max_freq = new_freq;
                max_freq_char = char;
            } else if (new_freq <= min_freq || char == min_freq_char) {
                min_freq = new_freq;
                min_freq_char = char;
            }
        }
        max_freq - min_freq
    }

    pub fn part1(input: &str) -> usize {
        let (poly, rules) = parse(input);
        let final_poly = Self::apply_n(poly, rules, 10);
        Self::score(&final_poly)
    }

    pub fn part2(input: &str) -> usize {
        let (poly, rules) = parse(input);
        let final_poly = Self::apply_n(poly, rules, 40);
        Self::score(&final_poly)
    }

    pub fn expand(chunk: &str, rules: &[Rule]) -> String {
        let mut pair = chunk.chars().take(2).peekable();
        let c1 = *pair.peek().unwrap();
        let c2 = *pair.peek().unwrap();
        let mut expanded: String = vec![c1].into_iter().collect();
        for rule in rules {
            if rule.from == vec![c1, c2].into_iter().collect::<String>() {
                expanded.push(rule.insert.chars().next().unwrap());
                break;
            }
        }
        expanded.push(c2);
        expanded
    }

    pub fn expand_n(mut self, chunk: &str, rules: &[Rule], n: usize) -> String {
        let mut found: Option<(usize, &str)> = None;
        for i in (1..=n).into_iter().rev() {
            if self.dp.get(&n).is_none() {
                self.dp.insert(n, HashMap::new());
            }
            if let Some(expanded) = self.dp.get(&n).unwrap().get(chunk) {
                found = Some((i, expanded));
                break;
            }
        }
        let init_level = found.unwrap_or((0, chunk)).0;
        let mut curr_str = found.unwrap_or((0, chunk)).1.to_string();
        for new_level in init_level + 1..=n {
            curr_str = Self::expand(&curr_str, rules);
            let mut expansions = self.dp.get_mut(&new_level).unwrap();
            expansions.insert(chunk.to_string(), curr_str.to_string());
        }
        self.dp.get(&n).unwrap().get(chunk).unwrap().to_string()
    }

    pub fn apply(poly: &str, rules: &[Rule]) -> String {
        let mut new_poly = String::new();

        for i in 0..poly.len() - 1 {
            let pair = poly.chars().skip(i).take(2).collect::<String>();
            // let mut chars = pair.chars().peekable();
            // let c1 = chars.next().unwrap();
            // let c2 = chars.peek().unwrap();
            new_poly.push(poly.chars().next().unwrap());
            for rule in rules {
                if rule.from == pair {
                    new_poly.push(pair.chars().next().unwrap());

                    // self.dp[Self::char_idx(c1)][Self::char_idx(*c2)][0] = "abc".to_string();
                    // new_poly.push(rule.insert.chars().next().unwrap());
                    break;
                }
            }
        }
        new_poly.push(poly.chars().last().unwrap());
        new_poly
    }
}
