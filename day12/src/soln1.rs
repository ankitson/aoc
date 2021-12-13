// use crate::shared;
use crate::shared::AdjList;
// use shared;
use std::collections::{HashSet, VecDeque};

// #[path = "shared.rs"]
// mod shared;

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

    ///Naive backtracking version
    pub fn part1(input: &str) -> u64 {
        let adj_list = crate::shared::parse(input);
        let mut npaths = 0u64;
        Self::backtrack(&adj_list, &mut vec!["start".to_string()], &mut npaths, &HashSet::new());
        npaths
    }

    pub fn backtrack(
        adj_list: &AdjList,
        mut current_path: &mut Vec<String>,
        npaths: &mut u64,
        visited: &HashSet<String>,
    ) {
        let tail = current_path.last().unwrap().clone();
        if tail == "end" {
            // println!("discovered path {:?}", current_path);
            *npaths += 1;
        }
        let mut new_visited = visited.clone();
        if (tail.to_uppercase() != *tail) {
            new_visited.insert(tail.clone());
        }
        for nbr in adj_list.nbrs.get(&tail).unwrap() {
            //try path current -> nbr
            if (!visited.contains(nbr)) {
                current_path.push(nbr.clone());
                Self::backtrack(adj_list, &mut current_path.clone(), npaths, &new_visited);
                current_path.pop();
            }
        }
    }

    pub fn part2(input: &str) -> Option<usize> {
        todo!()
    }
}
