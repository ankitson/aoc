use itertools::Itertools;
pub type Input<'a> = Vec<&'a [u8]>;
pub type Output = u64;

pub fn parse(bytes: &[u8]) -> Input {
    // let bytes = input.as_bytes();
    let skip_last = &bytes[0..bytes.len() - 1];
    let seqs = skip_last.split(|x| *x == b'\n' || *x == b',').collect_vec();
    seqs
}

/// Determine the ASCII code for the current character of the string.
/// Increase the current value by the ASCII code you just determined.
/// Set the current value to itself multiplied by 17.
/// Set the current value to the remainder of dividing itself by 256.
fn hash(str: &[u8]) -> u8 {
    let mut hash = 0u16;
    for c in str {
        hash += *c as u16;
        hash *= 17;
        hash = hash % 256;
    }
    hash as u8
}

pub fn run(raw_input: &[u8]) -> Output {
    let input = parse(raw_input);
    let mut total = 0;
    for str in input {
        let hashs = hash(str);
        total += hashs as u64;
    }
    total
}
