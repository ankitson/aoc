use std::cmp::Ordering;

use itertools::Itertools;

use crate::shared::{self, Output};
pub struct Soln1 {}
impl Soln1 {
    #[allow(dead_code)]
    pub fn read_all() -> usize {
        let contents: &str = include_str!("../../inputs/day07.txt");
        let mut i: usize = 0;
        for _ in contents.chars() {
            i += 1;
        }
        i
    }

    fn kind(hand: &Vec<char>, part2: bool) -> usize {
        let mut counts = hand.iter().counts(); //O(H)
        let mut max_non_j = 'F';
        if part2 {
            let j_freq = *counts.get(&'J').unwrap_or(&0);
            if j_freq > 0 {
                let mut max_non_j_freq = 0;
                for (card, cnt) in counts.iter() {
                    if !(**card == 'J') && *cnt > max_non_j_freq {
                        (max_non_j, max_non_j_freq) = (**card, *cnt)
                    }
                }
                counts.entry(&max_non_j).and_modify(|cnt| *cnt += j_freq);
                counts.remove(&'J');
                if max_non_j_freq == 0 {
                    //ALL Js
                    return 10;
                }
            }
        }
        let freqs = counts.values().sorted().rev().collect_vec(); //O(H LOG H)

        if freqs.len() == 1 {
            return 10; //FIVE OF A KIND
        } else if freqs.len() == 2 {
            if *freqs[0] == 4 {
                return 9; //(4,1) - FOUR OF A KIND
            } else if *freqs[0] == 3 {
                return 8; //(3,2) - FULL HOPUSE
            } else {
            }
        } else if freqs.len() == 3 {
            if *freqs[0] == 3 {
                return 7; //(3,1,1) - THREE OF A KIND
            }
            if *freqs[0] == 2 && *freqs[1] == 2 {
                return 6; //(2,2,1) - TWO PAIR
            }
        } else if freqs.len() > 3 {
            if *freqs[0] == 2 {
                return 5; //(2,1,1,1) - ONE PAIR
            }
        }
        4 //HIGH CARD
    }

    fn chton(c: char, p2: bool) -> u8 {
        if c == 'T' {
            b'0' + 10
        } else if c == 'J' {
            if !p2 {
                b'0' + 11
            } else {
                0
            }
        } else if c == 'Q' {
            b'0' + 12
        } else if c == 'K' {
            b'0' + 13
        } else if c == 'A' {
            b'0' + 14
        } else {
            c as u8
        }
    }

    fn compare(hand1: &Vec<char>, hand2: &Vec<char>, part2: bool) -> Ordering {
        let k1 = Self::kind(hand1, part2);
        let k2 = Self::kind(hand2, part2);

        if k1 > k2 {
            return Ordering::Greater;
        } else if k1 < k2 {
            return Ordering::Less;
        } else {
            let h1_nums = hand1.iter().map(|c| Self::chton(*c, part2)).collect_vec();
            let h2_nums = hand2.iter().map(|c| Self::chton(*c, part2)).collect_vec();
            return h1_nums.cmp(&h2_nums);
        }
    }

    pub fn part1(raw_input: &'static str) -> Output {
        let mut pairs = shared::parse(raw_input); //N pairs
                                                  //O(N LOG N)
        pairs.sort_by(|(h1, _r1), (h2, _r2)| Soln1::compare(h1, h2, false));
        let mut winnings = 0;
        //O(N)
        for (idx, (_hand, bid)) in pairs.iter().enumerate() {
            let mult = idx + 1;
            let winning = mult * bid;
            winnings += winning;
        }
        winnings
    }

    pub fn part2(raw_input: &'static str) -> Output {
        let mut pairs = shared::parse(raw_input);
        pairs.sort_by(|(h1, _r1), (h2, _r2)| Soln1::compare(h1, h2, true));

        let mut winnings = 0;
        for (idx, (_hand, bid)) in pairs.iter().enumerate() {
            let mult = idx + 1;
            let winning = mult * bid;
            winnings += winning;
        }

        winnings
    }
}
