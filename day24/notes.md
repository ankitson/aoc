clang-14 -O3 -emit-llvm -fno-discard-value-names -fsave-optimization-record prog.cpp -S -o prog.ll
clang++-14 -O3 -emit-llvm prog.cpp -c -o prog.bc
llvm-opt-report-14 prog.opt.yaml  > opt-report.txt
