use itertools::Itertools;
use regex::Regex;

use nom::bytes::complete::{tag, take_while1};
use nom::sequence::separated_pair;
use nom::{multi::separated_list1, IResult};

use anyhow::{anyhow, Result};

pub type Input<'a> = Vec<(&'a str, &'a str)>;
pub type Output = usize;

pub fn parse(raw_input: &'static str) -> Result<Input> {
    let ok_result = separated_list1(
        tag::<&str, &str, nom::error::Error<&str>>("\n\n"),
        separated_pair(take_while1(|s| s != '\n'), tag("\n"), take_while1(|s| s != '\n')),
    )(raw_input)?;
    Ok(ok_result.1)
}
