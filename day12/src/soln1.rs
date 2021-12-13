use crate::shared::AdjList;
use std::collections::{HashSet, VecDeque};

#[path = "shared.rs"]
mod shared;

pub struct Soln1 {}
impl Soln1 {
    pub fn npaths_from(start: String, adj_list: &AdjList) -> u64 {
        let mut to_visit: VecDeque<(String, String)> = VecDeque::from([(start, "none".to_string())]);
        let mut visited_edges: HashSet<(String, String)> = HashSet::new();
        let mut npaths = 0u64;
        while !to_visit.is_empty() {
            let (visit, from) = to_visit.pop_back().unwrap();
            if visit == "end" {
                npaths += 1;
            }
            let empty: Vec<String> = vec![];
            for nbr in adj_list.nbrs.get(&visit).unwrap_or(&empty) {
                if !visited_edges.contains(&(nbr.to_string(), visit.to_string())) {
                    to_visit.push_front((nbr.to_string(), visit.to_string()));
                }
            }
            visited_edges.insert((visit.to_string(), from.to_string()));
            if (visit.to_lowercase() == visit && from.to_lowercase() == from) {
                visited_edges.insert((from.to_string(), visit.to_string()));
            }
        }
        npaths
    }
    pub fn part1(input: &str) -> u64 {
        let adj_list = shared::parse(input);
        let end_adjs = adj_list.nbrs.get("end").unwrap();
        let mut npaths = 0u64;
        for end_adj in end_adjs {
            npaths += Self::npaths_from(*end_adj, &adj_list);
        }
        npaths
    }

    pub fn part2(input: &str) -> Option<usize> {
        todo!()
    }
}
