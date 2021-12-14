use std::collections::HashMap;

use itertools::Itertools;

use crate::shared::{parse, Rule};

pub struct Soln1 {}
impl Soln1 {
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
        println!("max {} {} min {} {}", max_freq, max_freq_char, min_freq, min_freq_char);
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

    pub fn apply(poly: &str, rules: &[Rule]) -> String {
        let mut new_poly = String::new();

        for i in 0..poly.len() - 1 {
            let pair = poly.chars().skip(i).take(2).collect::<String>();
            new_poly.push(pair.chars().next().unwrap());
            for rule in rules {
                if rule.from == pair {
                    new_poly.push(rule.insert.chars().next().unwrap());
                    break;
                }
            }
        }
        new_poly.push(poly.chars().last().unwrap());
        new_poly
    }
}
