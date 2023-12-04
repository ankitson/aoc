use itertools::Itertools;

use crate::shared::{parse, parse_faster, Input, Output};
pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part1_core(&input)
    }

    pub fn part1_core(cards: &Input) -> Output {
        let mut points = 0;
        for (wins, haves) in cards {
            let overlap = haves.iter().filter(|h| wins.contains(h)).count();
            if overlap > 0 {
                points += ((2 as u32).pow((overlap - 1) as u32)) as usize;
            }
        }
        points
    }

    pub fn part1_streamparse(raw_input: &str) -> Output {
        let lines = parse_faster(raw_input);
        let mut points = 0;
        for (wins, haves) in lines {
            let collect_wins = wins.collect_vec();
            let overlap = haves.filter(|h| collect_wins.contains(h)).count();
            if overlap > 0 {
                points += ((2 as u32).pow((overlap - 1) as u32)) as usize;
            }
        }
        points
    }

    pub fn part2_streamparse(raw_input: &str) -> Output {
        let mut lines = parse_faster(raw_input);
        let mut card_counts = vec![1 as usize; 200];
        let mut true_count = 0;
        for i in 0..card_counts.len() {
            if let Some((wins, haves)) = lines.next() {
                let count = card_counts[i];
                let collect_wins = wins.collect_vec();
                let overlap = haves.filter(|h| collect_wins.contains(h)).count();
                for j in 0..overlap {
                    card_counts[i + j + 1] += count;
                }
                true_count += 1;
            } else {
                break;
            }
        }
        card_counts[..true_count].iter().sum()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(cards: &Input) -> Output {
        let mut card_counts = vec![1 as usize; cards.len()];
        for i in 0..card_counts.len() {
            let count = card_counts[i];
            let (wins, haves) = &cards[i];
            let overlap = haves.iter().filter(|h| wins.contains(*h)).count();
            for j in 0..overlap {
                card_counts[i + j + 1] += count
            }
        }
        card_counts.iter().sum()
    }
}
