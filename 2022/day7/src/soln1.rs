use std::collections::{HashMap, HashSet, VecDeque};

use itertools::{Either, Itertools};

use crate::shared::{self, Listing};

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(input: &str) -> i32 {
        Self::part1_core(shared::parse(input))
    }

    pub fn part1_core(input: shared::Session) -> i32 {
        println!("Session:\n{:?}", input);

        //1. Build a hashmap from node -> children
        //   Node = Dir(Vec<Node>, usize) | File(usize)
        //2. Traverse the tree postorder and populate the sizes

        // store tree flattened as (key -> (children keys, size))
        // let mut tree: HashMap<Vec<String>, Either<(Vec<String>, usize), (String, usize)>> = HashMap::new();
        let mut tree: HashMap<Vec<String>, Either<Listing, Vec<Listing>>> = HashMap::new();
        let mut current_dir: Vec<String> = vec!["/".to_string()];
        tree.insert(current_dir.clone(), Either::Right(vec![])); //Either::Left((vec![], 0)));
        for (command, output) in input.into_iter() {
            dbg!(&command);
            dbg!(&output);
            match command {
                shared::Command::Cd(path) => match path.as_str() {
                    ".." => {
                        current_dir.pop();
                    }
                    "/" => current_dir = vec!["/".to_string()],
                    x => current_dir.push(x.to_string()),
                },
                shared::Command::Ls => {
                    for listing in output.as_ref().unwrap() {
                        match listing {
                            file @ Listing::File(name, size) => {
                                let mut child_path = current_dir.clone();
                                child_path.push(name.clone());
                                tree.insert(child_path, Either::Left(file.clone()));
                            }
                            dir @ Listing::Dir(name) => {
                                let mut child_path = current_dir.clone();
                                child_path.push(name.clone());
                                tree.insert(child_path.clone(), Either::Right(vec![dir.clone()]));
                            }
                        }
                    }
                    let value = tree.get_mut(&current_dir);
                    value.unwrap().as_mut().right().unwrap().extend(output.unwrap());
                    // let mut current_node = tree.get_mut(&current_dir).unwrap().right().unwrap();
                    // current_node.extend(output.unwrap());
                }
            };
        }

        dbg!(&tree);

        let mut sizes = HashMap::new();
        Self::populate(&mut tree, &mut sizes, &vec!["/".to_string()]);

        dbg!(&sizes);

        panic!("part1")
    }

    //must be called on a directory
    fn populate(
        tree: &mut HashMap<Vec<String>, Either<Listing, Vec<Listing>>>,
        sizes: &mut HashMap<Vec<String>, usize>,
        path: &Vec<String>,
    ) -> usize {
        // let mut sizes = HashMap::<Vec<String>, usize>::new();
        println!("populate {:?}", path);
        let node = tree.get(path).unwrap().clone();
        let mut dir_size = 0;
        match node {
            Either::Left(Listing::File(name, size)) => panic!("AHHHH"),
            Either::Left(Listing::Dir(name)) => panic!("ahhhh!!!"),
            Either::Right(listings) => {
                for listing in listings {
                    match listing {
                        Listing::File(name, size) => dir_size += size,
                        Listing::Dir(name) => {
                            println!("dir {:?} ", name);
                            let mut fpath = path.clone();
                            fpath.push(name.clone());
                            dir_size += Self::populate(tree, sizes, &fpath);
                        }
                    }
                }
            }
        }
        sizes.insert(path.clone(), dir_size);
        dir_size
    }

    // fn populate(tree: &mut HashMap<Vec<String>, Either<(Vec<String>, usize), (String, usize)>>, path: &Vec<String>) {
    //     let x = tree.get_mut(path).unwrap();
    //     match x {
    //         Either::Left((path, dirsize)) => {
    //             let subsizes =
    //         }
    //     }

    //     unimplemented!()
    // }

    pub fn part2(input: &str) -> i32 {
        panic!("part2");
        // Self::part2_core(shared::parse(input))
    }

    pub fn part2_core(input: &str) -> i32 {
        let mut seen: VecDeque<char> = VecDeque::from([]);
        for i in 0..input.len() {
            let char = input.chars().nth(i).unwrap();
            if seen.len() < 14 {
                seen.push_back(char);
                continue;
            }
            seen.pop_front().unwrap();
            seen.push_back(char);
            let mut dup = false;
            for i in 0..seen.len() {
                for j in i + 1..seen.len() {
                    if seen[i] == seen[j] {
                        dup = true;
                    }
                }
            }
            if !dup {
                return (i + 1).try_into().unwrap();
            }
        }
        panic!("AHHH");
    }

    pub fn part2_windows(input: &str) -> usize {
        input.as_bytes().windows(14).position(|w| -> bool { HashSet::<_>::from_iter(w.iter()).len() == 14 }).unwrap()
            + 14
    }

    pub fn part2_set(input: &str) -> i32 {
        let mut seen: VecDeque<char> = VecDeque::from([]);
        let mut seen_set: HashSet<char> = HashSet::with_capacity(14);
        for i in 0..input.len() {
            let char = input.chars().nth(i).unwrap();
            if seen.len() < 14 {
                seen.push_back(char);
                seen_set.insert(char);
                continue;
            }
            if seen_set.len() == 14 {
                return (i + 1).try_into().unwrap();
            }
            println!("ch = {} setlen = {}", i, seen_set.len());
            let oldest = seen.pop_front().unwrap();
            seen.push_back(char);
            //BUG: this will always remove oldest from the set, even if it reoccurs later in the seen vector
            //so the set ends up being smaller than it should.
            let removed = seen_set.remove(&oldest);
            seen_set.insert(char);
        }
        panic!("AHHH");
    }
}
