#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::iter::Copied;
mod binary {
    fn fmt_bits(num: u8) -> String {
        let bytes = num.to_be_bytes();
        let mut chars: Vec<char> = Vec::new();
        for mut byte in bytes {
            for bit in 0..8 {
                let char = if (byte & 1 == 1) { '1' } else { '0' };
                chars.push(char);
                byte >>= 1;
            }
        }
        chars.iter().rev().collect()
    }

    #[cfg(test)]
    mod tests {
        use super::fmt_bits;
        #[test]
        fn test_fmt_bits() {
            let num8: u8 = 0b00111111;
            let num16: u16 = 0b00111111;
            let num32: u32 = 0b00111111;
            let num64: u64 = 0b00111111;
            assert_eq!(fmt_bits(num8), "00111111");
        }
    }
}

pub fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

mod conv {
    #[derive(Debug)]
    enum ConversionError {
        Error,
    }

    struct MyVec<T>(Vec<T>);

    impl<T: TryInto<u32> + Copy> TryFrom<MyVec<T>> for u128 {
        type Error = ConversionError;

        fn try_from(vec: MyVec<T>) -> Result<Self, Self::Error> {
            let mut chars = vec![];
            for d in vec.0.iter() {
                if let Ok(u) = (*d).try_into() {
                    if let Some(ch) = char::from_digit(u, 10) {
                        chars.push(ch)
                    } else {
                        return Err(ConversionError::Error);
                    }
                } else {
                    return Err(ConversionError::Error);
                }
            }
            match chars.iter().collect::<String>().parse::<u128>() {
                Ok(n) => Ok(n),
                Err(pi) => Err(ConversionError::Error),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::MyVec;

        #[test]
        fn test_conv() {
            let digits: Vec<i32> = vec![1, 2, 3, 4];
            let n: u128 = MyVec(digits).try_into().unwrap();
            assert_eq!(n, 1234);

            let digits: Vec<u32> = vec![1, 2, 3, 4];
            let n: u128 = MyVec(digits).try_into().unwrap();
            assert_eq!(n, 1234);

            let digits: Vec<u64> = vec![1, 2, 3, 4];
            let n: u128 = MyVec(digits).try_into().unwrap();
            assert_eq!(n, 1234);

            let digits: Vec<i64> = vec![1, 2, 3, 4];
            let n: u128 = MyVec(digits).try_into().unwrap();
            assert_eq!(n, 1234);
        }
    }
}
