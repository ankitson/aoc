use itertools::Itertools;

pub type Input = (Vec<usize>, Vec<usize>);
pub type Output = usize;

//Time:      7  15   30
//Distance:  9  40  200

pub fn parse(input: &str) -> Input {
    let (times, dists) = input
        .lines()
        .map(|l| l.split_ascii_whitespace().skip(1).map(|x| x.parse::<usize>().unwrap()).collect_vec())
        .collect_tuple()
        .unwrap();
    return (times, dists);
}

pub fn parse_fast(input: &str) -> (Vec<usize>, Vec<usize>) {
    let bytes = input.as_bytes();
    let mut n = 0;
    let mut ts = Vec::with_capacity(10);
    let mut ds = Vec::with_capacity(10);
    let mut prev_spc = false;
    let mut i = 13;
    let mut b = bytes[i];

    while b != b'\n' {
        match b {
            b if b >= b'0' && b <= b'9' => {
                if prev_spc {
                    ts.push(n);
                    n = 0;
                    prev_spc = false;
                }
                n = 10 * n + ((b - b'0') as usize)
            }
            b' ' => {
                prev_spc = true;
            }
            _ => {}
        }
        i += 1;
        b = bytes[i];
    }
    ts.push(n);
    n = 0;
    i += 1 + 12;
    b = bytes[i];
    while b != b'\n' {
        match b {
            b if b >= b'0' && b <= b'9' => {
                if prev_spc {
                    ds.push(n);
                    n = 0;
                    prev_spc = false;
                }
                n = 10 * n + ((b - b'0') as usize)
            }
            b' ' => {
                prev_spc = true;
            }
            _ => {}
        }
        i += 1;
        if i >= bytes.len() {
            break;
        }
        b = bytes[i];
    }
    ds.push(n);
    (ts, ds)
}
