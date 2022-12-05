use crate::shared::{parse, AdjList};
use std::collections::HashMap;

pub struct Soln1 {}
impl Soln1 {
    ///Naive backtracking version
    pub fn part1(input: &str) -> u64 {
        let adj_list = parse(input);
        let mut npaths = 0u64;
        Self::backtrack(
            &adj_list,
            &mut vec!["start".to_string()],
            &mut npaths,
            &HashMap::from_iter(vec![("start".to_string(), 2)]),
            false,
        );
        npaths
    }

    pub fn backtrack(
        adj_list: &AdjList,
        current_path: &mut Vec<String>,
        npaths: &mut u64,
        visited: &HashMap<String, usize>,
        part2: bool,
    ) {
        let tail = current_path.last().unwrap().clone();
        if tail == "end" {
            *npaths += 1;
            return;
        }
        for nbr in adj_list.nbrs.get(&tail).unwrap() {
            let mut new_visited = visited.clone();
            if part2 {
                //can i visit this nbr?
                //if CAPS yes
                let is_caps = nbr.to_uppercase() == *nbr;
                if is_caps {
                    current_path.push(nbr.clone());
                    Self::backtrack(adj_list, &mut current_path.clone(), npaths, visited, part2);
                    current_path.pop();
                } else {
                    let visit_count = *visited.get(nbr).unwrap_or(&0);
                    let unvisited = visit_count == 0;
                    let visited_lt_twice = visit_count == 1;
                    let anything_else_visited_twice = visited
                        .iter()
                        .filter(|(k, v)| **v >= 2 && **k != *nbr && **k != "start")
                        .count()
                        > 0;
                    //NOTE: bug was here - i did not have the unvisited || part, so if
                    //anything vas visited twice, no new nodes would be visited at all..
                    if unvisited || (visited_lt_twice && !anything_else_visited_twice) {
                        current_path.push(nbr.clone());
                        if nbr != "end" {
                            let entry = new_visited.entry(nbr.to_string());
                            entry.or_insert(0);
                            *new_visited.get_mut(nbr).unwrap_or(&mut 0) += 1;
                        }
                        Self::backtrack(adj_list, &mut current_path.clone(), npaths, &new_visited, part2);
                        current_path.pop();
                        //new_visited is reset in loop
                    }
                }
            } else if !visited.contains_key(nbr) {
                current_path.push(nbr.clone());
                if nbr.to_uppercase() != *nbr {
                    new_visited.insert(nbr.clone(), 1);
                }
                Self::backtrack(adj_list, &mut current_path.clone(), npaths, &new_visited, part2);
                current_path.pop();
            }
        }
    }

    pub fn part2(input: &str) -> u64 {
        let adj_list = parse(input);
        let mut npaths = 0u64;
        Self::backtrack(
            &adj_list,
            &mut vec!["start".to_string()],
            &mut npaths,
            &HashMap::from_iter(vec![("start".to_string(), 2)]),
            true,
        );
        npaths
    }
}
