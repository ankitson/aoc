use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;

pub type Input = (HashSet<String>, HashSet<(String, String)>);
pub type Output = String;

fn parse(input: &str) -> Input {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let source = parts[0].trim().to_string();
        let destinations: HashSet<String> = parts[1].trim().split_whitespace().map(|s| s.to_string()).collect();
        nodes.insert(source.clone());
        for dest in &destinations {
            nodes.insert(dest.clone());
            if source < *dest {
                edges.insert((source.clone(), dest.clone()));
            } else {
                edges.insert((dest.clone(), source.clone()));
            }
        }
    }
    (nodes, edges)
}

fn find_path(
    graph: &Input,
    start: &str,
    end: &str,
    used_edges: &HashSet<(String, String)>,
) -> Option<Vec<(String, String)>> {
    let (nodes, edges) = graph;
    let mut visited = HashSet::new();
    let mut visit = VecDeque::new();
    let mut came_from: HashMap<String, (String, String)> = HashMap::new(); //node -> edge that reached it
    visit.push_back(start.to_string());
    visited.insert(start.to_string());

    while let Some(current) = visit.pop_front() {
        if current == end {
            // Reconstruct path
            let mut path = Vec::new();
            let mut node = current;
            while node != start {
                let edge = came_from.get(&node).unwrap();
                path.push(edge.clone());
                node = if edge.0 == node { edge.1.clone() } else { edge.0.clone() };
            }
            path.reverse();
            return Some(path);
        }

        for edge in edges {
            if used_edges.contains(edge) {
                continue;
            }

            // Find the neighbor node through this edge
            let neighbor = if edge.0 == current {
                edge.1.as_str()
            } else if edge.1 == current {
                edge.0.as_str()
            } else {
                continue;
            };

            if !visited.contains(neighbor) {
                visited.insert(neighbor.to_string());
                came_from.insert(neighbor.to_string(), edge.clone());
                visit.push_back(neighbor.to_string());
            }
        }
    }
    None
}

fn find_disjoint_paths(graph: &Input, start: &str, end: &str, count: usize) -> Option<Vec<Vec<(String, String)>>> {
    let mut used = HashSet::new();
    let mut paths = Vec::new();

    while let Some(path) = find_path(graph, start, end, &used) {
        paths.push(path.clone());
        used.extend(path);

        if paths.len() == count {
            return Some(paths);
        }
    }

    // If we didn't find enough paths, return None
    if paths.len() < count {
        None
    } else {
        Some(paths)
    }
}

pub fn part1(raw_input: &str) -> Output {
    let input = parse(raw_input);

    format!("{:?}", input)
}

pub fn part2(raw_input: &str) -> Output {
    unimplemented!()
}
