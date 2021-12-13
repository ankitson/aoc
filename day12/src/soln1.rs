use crate::shared::AdjList;
use std::collections::{HashSet, VecDeque};

#[path = "shared.rs"]
mod shared;

pub struct Soln1 {}
impl Soln1 {
    /*
        start
       /  |  \
      a . C   b
      npaths(start,end) = npaths(a,end, visit=any) + npaths(b,end, visit=any + npaths(C,end,visit=any-C)

      BFS, but when visiting a capital letter, remove it from the visited set.?
      increment npaths when end is reached

      start
       /\
    c-A--b-d
      \  /
       end
    */

    /*
      This returns 3 on the example not 10
      Once it visits b, it will never visit it again.
      In this example, it visits b before A.
      So it generates the 3 paths starting with b:

      start,b,A,c,A,end
      start,b,A,end
      start,B,end

      but misses out on the others starting with A. e.g:
      start,A,b,end

    */

    pub fn part1(input: &str) -> u64 {
        let adjlist = shared::parse(input);

        let mut current_path: Vec<String> = Vec::new();
        let mut to_visit: VecDeque<String> = VecDeque::from(["start".to_string()]);
        let mut visited: HashSet<String> = HashSet::new();
        let mut npaths = 0u64;
        while !to_visit.is_empty() {
            let visit = to_visit.pop_back().unwrap();
            current_path.push(visit.to_string());
            // println!("visiting {}", &visit);
            // println!("{:?}", &to_visit);
            if visit.to_uppercase() == visit {
                visited.remove(&visit);
            }
            if visit == "end" {
                current_path = Vec::new();
                npaths += 1;
            }
            let empty: Vec<String> = vec![];
            for nbr in adjlist.nbrs.get(&visit).unwrap_or(&empty) {
                if nbr == "end" || !visited.contains(nbr) {
                    to_visit.push_front(nbr.to_string());
                }
            }
            visited.insert(visit.to_string());
        }
        npaths
    }

    pub fn part2(input: &str) -> Option<usize> {
        todo!()
    }
}
