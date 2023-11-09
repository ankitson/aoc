use nom::branch::alt;
pub use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::error::Error;
use nom::Parser;
use nom::{multi, IResult};

mod parsers;
mod shared;
mod soln1;

#[macro_use]
extern crate scan_fmt;

#[cfg(feature = "heapprofile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {
    #[cfg(feature = "heapprofile")]
    let _profiler = dhat::Profiler::new_heap();

    parsers();

    println!("Hello Day 07!");
    let input: &str = include_str!("../../inputs/sample07.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/sample1 = {:?}", part1);

    let input: &str = include_str!("../../inputs/input07.txt");
    let part1 = soln1::Soln1::part1(input);
    println!("part1/day07 = {:?}", part1);

    let input: &str = include_str!("../../inputs/sample07.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/sample07 = \n{}", part2);

    let input: &str = include_str!("../../inputs/input07.txt");
    let part2 = soln1::Soln1::part2(input);
    println!("part2/day07 = \n{}", part2);
}

fn parse_input(input: &str) -> IResult<&str, &str> {
    //  note that this is really creating a function, the parser for abc
    //  vvvvv
    //         which is then called here, returning an IResult<&str, &str>
    //         vvvvv
    tag("abc")(input)
}

pub fn parsers() -> () {
    println!("Parsers with nom");
    let Ok((remaining_input, output)) = parsers::do_nothing_parser("my_input") else { todo!() };
    assert_eq!(remaining_input, "my_input");
    assert_eq!(output, "");

    let tag_parser = tag::<_, _, Error<_>>("abc");
    let Ok((rem, out)) = tag_parser("abcWorld") else { todo!() };
    assert_eq!(rem, "World");
    assert_eq!(out, "abc");

    assert!(tag_parser("defWorld").is_err());
    // $ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    use combine::parser::range::{range, take_while1};
    use combine::parser::repeat::sep_by;
    use combine::parser::Parser;

    //let wspc = multispace0;
    //let cmd_start = { |x: &str| tag::<_, _, Error<_>>("$ ")(x) };
    //#cursed language, terrible.
    // let cmd_name = alt((tag("ls"), tag("cd ")));
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_part1() {
        println!("Testing part1...");
        assert!(1 == 1)
    }
}
