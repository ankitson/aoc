clang-14 -O3 -emit-llvm -fno-discard-value-names -fsave-optimization-record prog.cpp -S -o prog.ll
clang++-14 -O3 -emit-llvm prog.cpp -c -o prog.bc
llvm-opt-report-14 prog.opt.yaml  > opt-report.txt

```rust
xadds = [10,13,15,-12,14,-2,13,-12,15,11,-3,-13,-12,-13];
yadds = [10, 5,12, 12, 6, 4,15,  3, 7,11, 2, 12,  4, 11];
zdivs = [ 1, 1, 1, 26, 1,26, 1, 26, 1, 1,26, 26, 26, 26];
for i in 0..14 {
  w = input[i];
  x = z % 26;
  x += xadds[i];
  if x != w {
    z *= 26
    z /= zdivs[i];
  } 
  y = w + yadds[i] * x;
  z += y;
}
```