use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Clone)]
pub enum Listing {
    File(String, usize),
    Dir(String),
}

pub(crate) type Session = Vec<(Command, Option<Vec<Listing>>)>;

pub fn parse(input: &str) -> Session {
    let mut pairs = vec![];
    let mut cmd: Option<Command> = None;
    let mut output: Option<Vec<Listing>> = None; //vec![];
    for line in input.lines() {
        if line.starts_with("$") {
            if cmd.is_some() {
                pairs.push((cmd.unwrap(), output));
                cmd = None;
                output = None; // ![];
            }
        }
        if line.starts_with("$ cd") {
            let dir = scan_fmt!(line, "$ cd {}", String).unwrap();
            cmd = Some(Command::Cd(dir));
        } else if line.starts_with("$ ls") {
            cmd = Some(Command::Ls);
        } else if line.starts_with("dir") {
            let dirname = line.split(" ").nth(1).unwrap();
            let out = Listing::Dir(dirname.to_string());
            if output.is_none() {
                output = Some(vec![]);
            }
            output.as_mut().map(|mut l| l.push(out));
        } else {
            let (size, name) = scan_fmt!(line, "{d} {}", usize, String).unwrap();
            let out = Listing::File(name, size);
            if output.is_none() {
                output = Some(vec![]);
            }
            output.as_mut().map(|mut l| l.push(out));
        }
    }
    if cmd.is_some() {
        pairs.push((cmd.unwrap(), output));
    }
    pairs
}
