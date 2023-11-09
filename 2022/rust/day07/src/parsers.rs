
use nom::IResult;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

//pub fn tag_parser(input: &str) -> IResult<&str, &str> {
//
//}
