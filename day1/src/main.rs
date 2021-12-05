mod soln;

/*
   BROKEN: Why does this not compile?
*/
pub fn main() {
    println!("Hello World");
    let contents: &str = include_str!("../day1.txt");
    let soln = soln::Soln1 {};
    soln.parse(contents);
}
