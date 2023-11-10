use itertools::Itertools;
use nom::IResult;
use std::collections::BTreeMap;

use nom::branch::alt;
pub use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::multispace0;
use nom::combinator::map;

use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, tuple};

use crate::shared::{Cd, Command, Input, Ls, Output};

pub struct Soln1 {}
impl Soln1 {
    fn wspc(x: &str) -> IResult<&str, &str> {
        multispace0(x)
    }
    fn symbol(x: &str) -> IResult<&str, &str> {
        take_while(|c: char| !c.is_whitespace())(x)
    }
    fn cmd_args(x: &str) -> IResult<&str, &str> {
        preceded(Self::wspc, Self::symbol)(x)
    }
    fn cmd(x: &str) -> IResult<&str, Command> {
        alt((
            map(tuple((tag("$ "), tag("cd"), Self::cmd_args)), |(_pref, _cmd, args)| Command::Cd(Cd(args))),
            map(tuple((tag("$ "), tag("ls"))), |(_, _)| Command::Ls(Ls)),
        ))(x)
    }
    fn dirlist_entry(x: &str) -> IResult<&str, (u32, &str)> {
        let (input, (size, name)) = alt((
            separated_pair(nom::character::complete::u32, tag(" "), Self::symbol),
            map(tuple((tag("dir "), Self::symbol)), |x| (0, x.1)),
        ))(x)?;
        Ok((input, (size, name)))
    }
    fn dirlist(x: &str) -> IResult<&str, Vec<(u32, &str)>> {
        separated_list0(tag("\n"), Self::dirlist_entry)(x)
    }

    pub fn parse_to_flatmap<'a>(
        input: &str,
    ) -> Result<BTreeMap<String, Vec<(u32, &str)>>, nom::Err<nom::error::Error<&str>>> {
        let mut input = input;
        let mut dir_path: Vec<&str> = vec![];
        let mut dirs: BTreeMap<String, Vec<(u32, &str)>> = BTreeMap::new();
        while input.len() > 0 {
            let (next_input, parsed) = delimited(Self::wspc, Self::cmd, Self::wspc)(input)?;
            input = next_input;

            match parsed {
                Command::Cd(Cd("/")) => {
                    dir_path = vec![""];
                }
                Command::Cd(Cd("..")) => {
                    dir_path.pop();
                }
                Command::Cd(Cd(dirname)) => {
                    dir_path.push(dirname);
                }
                Command::Ls(Ls) => {
                    let (next_input, parsed) = Self::dirlist(input)?;
                    input = next_input;
                    let current_path =
                        if dir_path.len() == 1 { "/".to_string() } else { dir_path.clone().into_iter().join("/") };
                    let files = parsed.into_iter().filter(|&(sz, _)| sz > 0).collect_vec();
                    dirs.entry(current_path).or_insert(files);
                }
            }
        }
        Ok(dirs)
    }

    fn gen_sizemap(dirs: BTreeMap<String, Vec<(u32, &str)>>) -> BTreeMap<String, u32> {
        let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
        for (path, files) in dirs.iter().sorted() {
            let size = files.iter().map(|(sz, _)| *sz).sum::<u32>(); //69
            let mut current_path = String::from("/");
            *sizes.entry(current_path.clone()).or_default() += size;
            for component in path.split('/').filter(|c| !c.is_empty()) {
                current_path =
                    if current_path == "/" { format!("/{component}") } else { format!("{current_path}/{component}") };
                *sizes.entry(current_path.clone()).or_default() += size
            }
        }
        sizes
    }

    pub fn part1_core<'a>(dirs: Input<'a>) -> Output {
        let sizes = Self::gen_sizemap(dirs);
        let matching = sizes.iter().filter_map(|(_, sz)| if *sz < 100_000 { Some(*sz) } else { None }).sum::<u32>();
        matching
    }

    pub fn part1(raw_input: &str) -> Output {
        let dirs = Self::parse_to_flatmap(raw_input).unwrap();
        Self::part1_core(dirs)
    }

    pub fn part2(raw_input: &str) -> Output {
        let dirs = Self::parse_to_flatmap(raw_input).unwrap();
        Self::part2_core(dirs)
    }

    pub fn part2_core<'a>(dirs: Input<'a>) -> Output {
        let sizes = Self::gen_sizemap(dirs);

        let total_space: u32 = 70_000_000;
        let desired_free: u32 = 30_000_000;
        let current_used: u32 = *sizes.get("/").unwrap_or(&0);
        let current_free: u32 = total_space - current_used;
        let need_free: u32 = desired_free.saturating_sub(current_free);
        if need_free == 0 {
            return 0;
        }

        let matching = sizes.iter().filter_map(|(_k, v)| if *v >= need_free { Some(*v) } else { None }).min().unwrap();
        matching
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{Cd, Command};
    use crate::soln1;
    #[test]
    fn test_part1() {
        println!("Testing part1...");

        let str1 = "$ cd dirname";
        let parse = soln1::Soln1::cmd(str1);
        assert!(parse.is_ok());

        let str2 = "$ ls";
        let parse2 = soln1::Soln1::cmd(str2);
        assert!(parse2.is_ok());
    }
}
