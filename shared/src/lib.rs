use std::fmt::Display;

/*
    BROKEN: This implementation does not compile because the returned string slice has an unknown lifetime
*/
trait Solution<InputType, OutputType> {
    fn parse(input: &str) -> InputType;

    fn unparse(output: OutputType) -> &str;

    fn part1_core(input: InputType) -> OutputType;

    fn part2_core(input: InputType) -> OutputType;

    fn part1(input: &str) -> &str {
        Self::unparse(Self::part1_core(Self::parse(input)))
    }

    fn part2(input: &str) -> &str {
        Self::unparse(Self::part2_core(Self::parse(input)))
    }
}
