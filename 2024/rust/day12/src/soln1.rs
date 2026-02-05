use core::num;
use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub type Input = Vec<Vec<char>>;
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    input.lines().map(|row| row.chars().collect_vec()).collect_vec()
}

pub fn part1(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut visited = HashSet::new();
    let mut total = 0;
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if !visited.contains(&(ri, ci)) {
                let (mut area, mut perim) = (0, 0);
                dfs(&grid, ri, ci, &mut area, &mut perim, &mut visited);
                let contrib = area * perim;
                println!("For {ri},{ci}, area = {area}, perim = {perim} contrib={contrib}");
                total += contrib
            }
        }
    }
    total
}

fn dfs(grid: &Input, ri: usize, ci: usize, area: &mut usize, perim: &mut usize, visited: &mut HashSet<(usize, usize)>) {
    if visited.contains(&(ri, ci)) {
        return;
    }
    visited.insert((ri, ci));
    let mut perim_contrib = 4;
    if ri > 0 && grid[ri - 1][ci] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ri < grid.len() - 1 && grid[ri + 1][ci] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ci > 0 && grid[ri][ci - 1] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ci < grid[0].len() - 1 && grid[ri][ci + 1] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    println!("{ri} {ci} contributes {perim_contrib}");
    *perim += perim_contrib;
    *area += 1;
    for nbr in util::grid::nbrs4(ri, ci, grid.len(), grid[0].len()) {
        if !visited.contains(&nbr) && grid[nbr.0][nbr.1] == grid[ri][ci] {
            dfs(&grid, nbr.0, nbr.1, area, perim, visited)
        }
    }
}

fn dfs3(
    grid: &Input,
    ri: usize,
    ci: usize,
    area: &mut usize,
    // nsides: &mut usize,
    // last_dir: (isize, isize),
    edges: &mut Vec<((usize, usize), (usize, usize))>,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(ri, ci)) {
        return;
    }
    visited.insert((ri, ci));
    // let mut perim_contrib = 4;
    // if ri > 0 && grid[ri - 1][ci] == grid[ri][ci] {
    //     perim_contrib -= 1;
    // }
    // if ri < grid.len() - 1 && grid[ri + 1][ci] == grid[ri][ci] {
    //     perim_contrib -= 1;
    // }
    // if ci > 0 && grid[ri][ci - 1] == grid[ri][ci] {
    //     perim_contrib -= 1;
    // }
    // if ci < grid[0].len() - 1 && grid[ri][ci + 1] == grid[ri][ci] {
    //     perim_contrib -= 1;
    // }
    // println!("{ri} {ci} contributes {perim_contrib}");
    // *perim += perim_contrib;
    *area += 1;
    for nbr in util::grid::nbrs4(ri, ci, grid.len(), grid[0].len()) {
        if !visited.contains(&nbr) && grid[nbr.0][nbr.1] == grid[ri][ci] {
            let dir = (nbr.0 as isize - ri as isize, nbr.1 as isize - ci as isize);
            edges.push(((ri, ci), (nbr.0, nbr.1)));
            // if dir != last_dir {
            // *nsides += 1;
            // }
            dfs3(&grid, nbr.0, nbr.1, area, edges, visited)
        }
    }
}

fn dfs2(
    grid: &Input,
    ri: usize,
    ci: usize,
    num_nodes: &mut usize,
    perim: &mut usize,
    // prev: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(ri, ci)) {
        return;
    }
    visited.insert((ri, ci));
    let mut perim_contrib = 4;
    if ri > 0 && grid[ri - 1][ci] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ri < grid.len() - 1 && grid[ri + 1][ci] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ci > 0 && grid[ri][ci - 1] == grid[ri][ci] {
        perim_contrib -= 1;
    }
    if ci < grid[0].len() - 1 && grid[ri][ci + 1] == grid[ri][ci] {
        perim_contrib -= 1;
    }

    *perim += perim_contrib;
    let mut num_nodes_contrib = 4;
    if ri > 0 && grid[ri - 1][ci] == grid[ri][ci] {
        num_nodes_contrib -= 2;
        if ci > 0 && grid[ri][ci - 1] == grid[ri][ci] {
            num_nodes_contrib -= 1;
        }
    } else if ci > 0 && grid[ri][ci - 1] == grid[ri][ci] {
        num_nodes_contrib -= 2;
        if ri > 0 && grid[ri - 1][ci] == grid[ri][ci] {
            num_nodes_contrib -= 1;
        }
    }
    *num_nodes += num_nodes_contrib;
    println!("{ri} {ci} contributes perim={perim_contrib}, num_nodes={num_nodes_contrib}");

    for nbr in util::grid::nbrs4(ri, ci, grid.len(), grid[0].len()) {
        if !visited.contains(&nbr) && grid[nbr.0][nbr.1] == grid[ri][ci] {
            dfs2(&grid, nbr.0, nbr.1, perim, num_nodes, visited)
        }
    }
}

pub fn part2(raw_input: &str) -> Output {
    part2_dfs(raw_input)
}

/*
 AAAA
 BBCD
 ..

edge at idx (0,0) can be VER or HORZ
edge at idx(0,0) means to the LEFT and ABOVE grid idx (0,0)
*/
fn edges_to_num_sides(edges: &Vec<((usize, usize), (usize, usize))>) -> usize {
    0
}

pub fn part2_dfs(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut visited = HashSet::new();
    let mut total = 0;
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if !visited.contains(&(ri, ci)) {
                let mut edges = vec![];
                let mut num_nodes = 0;
                // dfs2(&grid, ri, ci, &mut area, &mut nsides, (100000, 100000), &mut visited);
                dfs3(&grid, ri, ci, &mut num_nodes, &mut edges, &mut visited);
                println!("For {ch}={ri},{ci}, num_nodes = {num_nodes}, edges={edges:?}");
                // println!("edges = ")
                // let contrib = num_nodes * nsides;
                // total += contrib;
            }
        }
    }
    total
}

pub fn part2_euler(raw_input: &str) -> Output {
    let grid = parse(raw_input);
    let mut visited = HashSet::new();
    let mut total = 0;
    for (ri, row) in grid.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            if !visited.contains(&(ri, ci)) {
                let (mut num_nodes, mut perim) = (0, 0);
                // dfs2(&grid, ri, ci, &mut area, &mut nsides, (100000, 100000), &mut visited);
                dfs2(&grid, ri, ci, &mut num_nodes, &mut perim, &mut visited);
                let faces = num_nodes - perim - 2;
                //area + perim - faces = 2
                let contrib = num_nodes * faces;
                println!("For {ri},{ci}, num_nodes = {num_nodes}, perim={perim}, faces = {faces} contrib={contrib}");
                total += contrib
            }
        }
    }
    total
}
