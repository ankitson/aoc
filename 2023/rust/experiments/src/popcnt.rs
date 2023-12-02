pub fn popcnt_naive(x: u32) -> u32 {
    let mut bits = [0; 32];
    for (i, bit) in bits.iter_mut().enumerate() {
        *bit = (x >> i) & 1;
    }
    bits.into_iter().fold(0, |total, bit| total + bit)
}

#[inline(never)]
pub fn popcnt_split(mut x: u32) -> u32 {
    let even = x & 0x55555555;
    let odd = x & 0xaaaaaaaa;

    x = even + (odd >> 1);

    let even = x & 0x33333333;
    let odd = x & 0xcccccccc;

    x = even + (odd >> 2);

    let even = x & 0x0f0f0f0f;
    let odd = x & 0xf0f0f0f0;

    x = even + (odd >> 4);

    let even = x & 0x00ff00ff;
    let odd = x & 0xff00ff00;

    x = even + (odd >> 8);

    let even = x & 0x0000ffff;
    let odd = x & 0xffff0000;

    x = even + (odd >> 16);

    x
}
#[cfg(test)]
mod tests {
    use crate::popcnt::{popcnt_naive, popcnt_split};

    #[test]
    fn test_popcnt() {
        let n1 = 0x10101010;
        let n2 = 0x11111111;
        let n3 = 0x00000000;
        let n4 = 0xaaaa1111;

        assert!(popcnt_naive(n1) == 4);
        assert!(popcnt_naive(n2) == 8);
        assert!(popcnt_naive(n3) == 0);
        assert!(popcnt_naive(n4) == (2 * 4 + 4));
        assert!(popcnt_split(n1) == 4);
        assert!(popcnt_split(n2) == 8);
        assert!(popcnt_split(n3) == 0);
        assert!(popcnt_split(n4) == (2 * 4 + 4));
    }
}
