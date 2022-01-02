#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
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

// pub fn vec_into(v: Vec<T>) -> Vec<U>
// where
//     T: TryInto<U>,
// {
//     v.into_iter().map(|x| x.try_into::<U>().unwrap()).collect::<Vec<U>>()
// }

pub fn combo(draw_from: Vec<isize>, target_length: usize, into: &mut Vec<Vec<isize>>) {
    combo_help(&draw_from, target_length, vec![], into);
}

fn combo_help(draw_from: &Vec<isize>, target_length: usize, current: Vec<isize>, into: &mut Vec<Vec<isize>>) {
    if current.len() == target_length {
        into.push(current);
        return;
    }

    for num in draw_from {
        let mut next = current.clone();
        next.push(*num);
        combo_help(draw_from, target_length, next, into);
    }
}

pub mod aoc {
    // use reqwest::{cookie, Client, ClientBuilder, Response, Url};
    use cookie_store::{Cookie, CookieStore};
    use std::{sync::Arc, time::Duration};
    use ureq::Agent;
    // use ureq::Cookie;

    const SESSION: &str =
        "53616c7465645f5f1a75ed6a2f81147d095d5166ad699642aa8a2a7b6e6a7c3a1f5bb2ec3ab81d562eba00d37189d57a";

    //sync rust.. also tragic
    fn fetch(day: usize) {
        // let cks = CookieStore::default();
        // let ck: Cookie = Cookie::parse(format!("session={}", SESSION), Url::parse("adv.com").unwrap()).unwrap();
        // let ck = Cookie::parse(
        //     format!(
        //         "{}{}",
        //         format!("session={}", SESSION),
        //         ";Expires=\"Mon, 01 Dec 2031 07:11:25 GMT\"",
        //     ),
        //     ".adventofcode.com".parse::<Url>(), //&url(url_str),
        // );
        // let ck = Cookie::new("session", SESSION);
        // // ck.insert(Cookie::new("session", SESSION), ".adventofcode.com");
        // let agent: Agent = ureq::AgentBuilder::new()
        //     .timeout_read(Duration::from_secs(5))
        //     .timeout_write(Duration::from_secs(5))
        //     .cookie_store(cks)
        //     .build();
    }

    //async rust... a tragic tale
    // pub async fn fetch_input(day: usize) -> Result<String, reqwest::Error> {
    //     let url = format!("https://adventofcode.com/2021/day/{}/input", day)
    //         .parse::<Url>()
    //         .unwrap();
    //     let cookie_jar = reqwest::cookie::Jar::default();
    //     cookie_jar.add_cookie_str(&format!("session={}; Domain=.adventofcode.com", SESSION), &url);
    //     let client = ClientBuilder::new()
    //         .cookie_provider(Arc::new(cookie_jar))
    //         .build()
    //         .unwrap();
    //     let response = client.get(url).send().await.unwrap();
    //     let text = response.text().await;
    //     text
    // }
}
