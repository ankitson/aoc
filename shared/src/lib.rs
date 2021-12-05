/*
    FIX1: This returns Strings, meaning the string allocated is moved to the caller

    This compiles, but what if say the output type is a String, and unparse should return a slice of that string only?
*/
pub trait Solution<InputType, OutputType> {
    fn parse(input: &str) -> InputType;

    fn unparse(output: OutputType) -> String;

    fn part1_core(input: InputType) -> OutputType;

    fn part2_core(input: InputType) -> OutputType;

    fn part1(input: &str) -> String {
        Self::unparse(Self::part1_core(Self::parse(input)))
    }

    fn part2(input: &str) -> String {
        Self::unparse(Self::part2_core(Self::parse(input)))
    }
}
