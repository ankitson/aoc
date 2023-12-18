magnus day 14

// hash function
fn splitmix64(mut x: u64) -> u64 {
    // http://xorshift.di.unimi.it/splitmix64.c
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x = x ^ (x >> 31);
    x
}

//get input
let input_p = a.as_ptr() as *const u8;
for i in 0..312 {
            let input_p = input_p.offset(i * 32);
            let input_reg = _mm256_loadu_si256(input_p as *const __m256i);
            let is_ball = _mm256_movemask_epi8(_mm256_cmpeq_epi8(balls, input_reg)) as u32;
            let is_wall = _mm256_movemask_epi8(_mm256_cmpeq_epi8(walls, input_reg)) as u32;
            let x = (is_ball as u64) << 32 | is_wall as u64;
            state = splitmix64(state ^ x);
        }
