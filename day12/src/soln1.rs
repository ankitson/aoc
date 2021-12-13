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
     * Another approach:
     * 
     * np(p,q) = number of paths(p,q) = sum ( number of paths (p, vi) * number of paths (vi, q) )
     *  
     * we want np(start,end) = sum (np(start,s1),np(start,s2),..,np(start,si)) si = nbr of start
     * 
     * calculate np for all pairs?
     * 
     * visit start
     *   visit A
     *      np(start,A) = 1
     *      visit c
     *          np(A,c) = 1
     *      np(A,A) = 1
     *      visit b
     *          np(A,b) = 1
     *          visit d
     *              np(b,d) = 1
     * 
     *          visit end
     *              np(b,end) = 1
     *          
     *          visit A
     *              np(b,A) = 2
     *      visit end
     *          np(A,end)      
     *          
     *          
     * 
     * 
     * 
     * 
     */


    pub fn part1(input: &str) -> u64 {
        todo!()
    }

    pub fn part2(input: &str) -> Option<usize> {
        todo!()
    }
}
