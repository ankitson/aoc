use std::cmp::Ordering;

use itertools::Itertools;

use crate::shared::{Input, Output};
pub struct Soln1 {}
impl Soln1 {
    fn kind(hand: &Vec<char>) -> usize {
        if hand.iter().all_equal() {
            return 10; //FIVE OF A KIND
        } else {
            let counts = hand.iter().counts();
            // println!("counts = {counts:?}");
            if counts.len() == 2 {
                let values = counts.values().sorted().rev().collect_vec();
                if *values[0] == 4 {
                    //4, 1
                    return 9; //FOUR OF A KIND
                } else if *values[0] == 3 {
                    //3,2
                    return 8; //FULL HOPUSE
                } else {
                }
            } else if counts.len() == 3 {
                let freqs = counts.values().sorted().rev().collect_vec();
                if *freqs[0] == 3 {
                    //3,1,1
                    return 7; //THREE OF A KIND
                }
                if *freqs[0] == 2 && *freqs[1] == 2 {
                    //2,2,1
                    return 6; //TWO PAIR
                }
            } else if counts.len() > 3 {
                let freqs = counts.values().sorted().rev().collect_vec();

                if *freqs[0] == 2 {
                    return 5; //ONE PAIR
                }
            }
        }
        4 //HIGH CARD
    }
    fn kind2(hand: &Vec<char>) -> usize {
        let mut counts = hand.iter().counts();
        let j_freq = *counts.get(&'J').unwrap_or(&0);
        let mut max_non_j = 'F';
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

        let freqs = counts.values().sorted().rev().collect_vec();

        let mut v = 4; //SINGLE
        if freqs.len() == 1 {
            v = 10;
        } else if freqs.len() == 2 {
            if *freqs[0] == 4 {
                v = 9; //(4,1) - FOUR OF A KIND
            } else if *freqs[0] == 3 {
                v = 8; //(3,2) - FULL HOUSE
            } else {
            }
        } else if freqs.len() == 3 {
            if *freqs[0] == 3 {
                v = 7; //(3,1,1) - THREE OF A KIND
            }
            if *freqs[0] == 2 && *freqs[1] == 2 {
                v = 6; //(2,2,1) - TWO PAIR
            }
        } else if freqs.len() > 3 {
            if *freqs[0] == 2 {
                v = 5; //(2,1,1,1) - ONE PAIR
            }
        }

        let ret = v; //(v + njoker).min(10);
        println!("hand: {hand:?}, njoker={j_freq} v ={v} kind = {ret}");
        ret
    }
    fn chton(c: char) -> u8 {
        if c == 'T' {
            b'0' + 10
        } else if c == 'J' {
            b'0' + 11
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
    fn chton2(c: char) -> u8 {
        if c == 'T' {
            b'0' + 10
        } else if c == 'J' {
            0
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
    fn ntoch(c: u8) -> char {
        if c == b'0' + 10 {
            'T'
        } else if c == b'0' + 11 {
            'J'
        } else if c == b'0' + 12 {
            'Q'
        } else if c == b'0' + 13 {
            'K'
        } else if c == b'0' + 14 {
            'A'
        } else {
            c as char
        }
    }

    fn compare(hand1: &Vec<char>, hand2: &Vec<char>) -> Ordering {
        let k1 = Self::kind(hand1);
        let k2 = Self::kind(hand2);

        let h1print = hand1.iter().cloned().join("");
        let h2print = hand2.iter().cloned().join("");

        if k1 > k2 {
            // println!("> Hand: {h1print:?} kind={k1} > h2: {h2print:?} kind={k2}");
            return Ordering::Greater;
        } else if k1 < k2 {
            // println!("< Hand: {h1print:?} kind={k1} < h2: {h2print:?} kind={k2}");
            return Ordering::Less;
        } else {
            let h1_nums = hand1.iter().map(|c| Self::chton(*c)).collect_vec();
            let h2_nums = hand2.iter().map(|c| Self::chton(*c)).collect_vec();
            return h1_nums.cmp(&h2_nums);
        }
    }

    fn compare2(hand1: &Vec<char>, hand2: &Vec<char>) -> Ordering {
        let k1 = Self::kind2(hand1);
        let k2 = Self::kind2(hand2);

        let h1print = hand1.iter().cloned().join("");
        let h2print = hand2.iter().cloned().join("");

        if k1 > k2 {
            // println!("> Hand: {h1print:?} kind={k1} > h2: {h2print:?} kind={k2}");
            return Ordering::Greater;
        } else if k1 < k2 {
            // println!("< Hand: {h1print:?} kind={k1} < h2: {h2print:?} kind={k2}");
            return Ordering::Less;
        } else {
            let h1_nums = hand1.iter().map(|c| Self::chton2(*c)).collect_vec();
            let h2_nums = hand2.iter().map(|c| Self::chton2(*c)).collect_vec();
            return h1_nums.cmp(&h2_nums);
        }
    }

    pub fn part1(raw_input: &str) -> Output {
        let lines = raw_input.split("\n");
        let mut pairs = lines
            .filter_map(|x| {
                if x.len() < 2 {
                    return None;
                }
                let mut sp = x.split_ascii_whitespace();
                let hand = sp.next().unwrap().chars().collect_vec();
                let rank = sp.next().unwrap().parse::<usize>().unwrap();
                Some((hand, rank))
            })
            .collect_vec();
        pairs.sort_by(|(h1, _r1), (h2, _r2)| Soln1::compare(h1, h2));
        let mut winnings = 0;
        for (idx, (hand, bid)) in pairs.iter().enumerate() {
            let hprint = hand.iter().join("");
            let mult = idx + 1;
            let winning = mult * bid;
            winnings += winning;
        }
        winnings
    }

    pub fn part1_core(_input: &Input) -> Output {
        todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let lines = raw_input.split("\n");
        let mut pairs = lines
            .filter_map(|x| {
                if x.len() < 2 {
                    return None;
                }
                let mut sp = x.split_ascii_whitespace();
                let hand = sp.next().unwrap().chars().collect_vec();
                let rank = sp.next().unwrap().parse::<usize>().unwrap();
                Some((hand, rank))
            })
            .collect_vec();
        // println!("{:?}", pairs);
        pairs.sort_by(|(h1, _r1), (h2, _r2)| Soln1::compare2(h1, h2));

        let h1 = "AJAAA".chars().collect_vec();
        let h2 = "JJJJJ".chars().collect_vec();
        // println!("{:?}", Self::compare2(&h1, &h2));

        // pairs.reverse();
        let mut winnings = 0;
        for (idx, (hand, bid)) in pairs.iter().enumerate() {
            let _hprint = hand.iter().join("");
            let mult = idx + 1;
            let winning = mult * bid;
            // println!("{hprint:?} ranks {mult}, wins {winning}");
            winnings += winning;
            // println!("winning for {hand:?}= {winning} = {mult}*{bid}");
        }
        if pairs.len() > 10 {
            let ranked = &pairs.iter().map(|x| x.0.clone().iter().join("")).collect_vec();
            println!("{ranked:?}");
            // println!("first 10: {:?}", &pairs.iter().map(|x| x.0.clone().iter().join("")).collect_vec()[0..10]);

            // println!(
            // ";ast: {:?}",
            // &pairs.iter().map(|x| x.0.clone().iter().join("")).collect_vec()[pairs.len() - 10..pairs.len()]
            // );
        }
        winnings
    }

    pub fn part2_core(_input: &Input) -> Output {
        todo!()
    }
}
