//discord : https://discord.com/channels/273534239310479360/386246790565068811/1181869800830550016
//magnus hegdahl:
//Part 1 - bsearch (1.0s) ...               31.667 ns/iter (0.999 R²)
//Part 2 - bsearch (1.0s) ...               44.166 ns/iter (1.000 R²)
fn solve<const WIDTH: usize>(time: i64, dist: i64) -> i64 {
    let test = |x: i64| dist < (time - x) * x;
    let test_left = |x: i64| 2 * x <= time && !test(x);

    let mut lo = 0;

    for bit in (0..WIDTH).rev() {
        let jmp = 1 << bit;
        if test_left(lo + jmp) {
            lo += jmp;
        }
    }

    time - 2 * lo - 1
}
