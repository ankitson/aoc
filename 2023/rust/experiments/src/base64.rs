use std::simd::Simd;

pub fn encode_base64(bytes: &[u8]) -> String {
    let mut chars = vec![];
    for byte_triple in bytes.chunks(3) {
        let len = byte_triple.len();
        let b0 = byte_triple[0];
        let b1 = if len > 1 { byte_triple[1] } else { 0 };
        let b2 = if len > 2 { byte_triple[2] } else { 0 };

        let t1 = (b0 & 0b11111100) >> 2;
        let t2 = (b0 & 0b00000011) << 4 | (b1 >> 4);
        let t3 = (b1 & 0b00001111) << 2 | (b2 >> 6);
        let t4 = b2 & 0b00111111;
        let chunks = [t1, t2, t3, t4];

        for (i, chunk) in chunks.iter().enumerate() {
            if i > len {
                chars.push('=');
            } else {
                chars.push(match chunk {
                    0..=25 => b'A' + chunk,
                    26..=51 => b'a' + (chunk - 26),
                    52..=61 => b'0' + (chunk - 52),
                    62 => b'+',
                    63 => b'/',
                    _ => panic!("illegal 6bit {chunk:08b} when encoding"),
                } as char);
            }
        }
    }
    chars.into_iter().collect()
}

pub fn decode_base64(string: &str) -> Vec<u8> {
    let bytes = string.as_bytes();
    let bytes = match &bytes[..] {
        [p @ .., b'=', b'='] | [p @ .., b'='] | p => p,
    };
    let mut curr = 0u32;
    let mut decoded = vec![];
    for chunk in bytes.chunks(4) {
        for char in chunk {
            //each char is 6 bits, 4 chars = 24 bits = 3 bytes
            let sixbits = match char {
                b'A'..=b'Z' => char - b'A',
                b'a'..=b'z' => (char - b'a') + 26,
                b'0'..=b'9' => (char - b'0') + 52,
                b'+' => 62,
                b'/' => 63,
                _ => panic!("illegal char {char} in base64 string"),
            };
            curr <<= 6;
            curr |= sixbits as u32;
        }
        curr <<= 32 - 6 * chunk.len();
        let msg_len: usize = (chunk.len() * 6) / 8;
        decoded.extend_from_slice(&curr.to_be_bytes()[..msg_len]);
    }
    decoded
}

fn decode_hot(ascii: Simd<u8, 4>) -> (Simd<u8, 4>, bool) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::Itertools;

    use once_cell::sync::Lazy;
    #[allow(dead_code)]
    //Don't know how to use this yet.
    static PAIRS: Lazy<Vec<(String, String)>> = Lazy::new(|| {
        vec![
            ("light work.".to_string(), "bGlnaHQgd29yay4A=".to_string()),
            ("light work".to_string(), "bGlnaHQgd29yaw==".to_string()),
            ("light wor".to_string(), "bGlnaHQgd29y".to_string()),
            ("light wo".to_string(), "bGlnaHQgd28=".to_string()),
            ("Many hands make light work.".to_string(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string()),
        ]
    });

    #[test]
    fn test_encode_base64() {
        let pairs = vec![
            ("light work.".to_string(), "bGlnaHQgd29yay4=".to_string()),
            ("light work".to_string(), "bGlnaHQgd29yaw==".to_string()),
            ("light wor".to_string(), "bGlnaHQgd29y".to_string()),
            ("light wo".to_string(), "bGlnaHQgd28=".to_string()),
            ("Many hands make light work.".to_string(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string()),
        ];
        for (text, encoded) in pairs {
            assert_eq!(encode_base64(&text.bytes().collect_vec()), encoded)
        }
    }

    #[test]
    fn test_decode_base64() {
        let pairs = vec![
            ("light work.".to_string(), "bGlnaHQgd29yay4=".to_string()),
            ("light work".to_string(), "bGlnaHQgd29yaw==".to_string()),
            ("light wor".to_string(), "bGlnaHQgd29y".to_string()),
            ("light wo".to_string(), "bGlnaHQgd28=".to_string()),
            ("Many hands make light work.".to_string(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string()),
        ];
        for (text, encoded) in pairs {
            assert_eq!(text.as_bytes(), decode_base64(&encoded))
        }
    }
}
