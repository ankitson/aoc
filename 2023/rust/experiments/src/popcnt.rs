pub fn popcnt(mut x: u32) -> u32 {
    let mut bits = [0; 32];
    for (i, bit) in bits.iter_mut().enumerate() {
        *bit = (x >> i) & 1;
    }
    bits.into_iter().fold(0, |total, bit| total + bit)
}
