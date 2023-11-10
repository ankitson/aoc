use std::collections::BTreeMap;

pub type Input<'a> = BTreeMap<String, Vec<(u32, &'a str)>>;
pub type Output = u32;

#[derive(Debug, Copy, Clone)]
pub struct Ls;
#[derive(Debug, Copy, Clone)]
pub struct Cd<'a>(pub &'a str);

#[derive(Debug, Copy, Clone)]
pub enum Command<'a> {
    Ls(Ls),
    Cd(Cd<'a>),
}
