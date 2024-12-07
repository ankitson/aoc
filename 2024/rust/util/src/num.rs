//bsearch
// for bit in (0..WIDTH).rev() {
//     let jmp = 1 << bit;
//     if test(hi + jmp) {
//         hi += jmp;
//     }
//}

pub fn concat_num(n: usize, m: usize) -> usize {
    m + n * 10usize.pow(m.ilog10() + 1)
}
