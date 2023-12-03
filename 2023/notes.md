# Advent of Code 2023

## Table of Contents

1. [Day 1](#day-1-trebuchet?!)
2. [Day 2](#day-2-tbd)

# Day 2: TBD

Profiling - `cargo flamegraph --bench corebench`. running outside the container, from host. before that do `sudo echo 0 | tee /proc/sys/kernel/kptr_restrict`. then you should not see any errors about ```
WARNING: Kernel address maps (/proc/{kallsyms,modules}) are restricted,
check /proc/sys/kernel/kptr_restrict and /proc/sys/kernel/perf_event_paranoid.

Samples in kernel functions may not be resolved if a suitable vmlinux
...

````
but the generated flamegraph also includes the setup code for criterion, and i can't actually see any of my code..

this may be useful: https://www.jibbow.com/posts/criterion-flamegraphs/


What seems to work is just running the binary from `flamegraph` instead of the bench (`cargo flamegraph -p day02 --bin day02`), and wrapping the main operations in a for loop to run them many times


# Day 1: Trebuchet?!

## Part 1

Part 1 was simple enough.

## Part 2

On part 2, i spent a little time fumbling with `matches` and `match_indices`. I assumed these would match regexes so I had something like `str.matches("one|two|three...")` but they don't. So i replaced that with a pile of 9 `matches`, one for each number. But it turns out we need the indices, so i found the `match_indices` function to do that.

I spent a fair bit of time debugging too - i was a bit confused on the logic for bounds checking the lengths and then one logic bug - i did not sort the combined matches array by index before looking at the first/last indices so it was picking random matches.

### Soln 2

I made a faster solution for part 2 using regexes. This one runs in 361.05µs compared to around double the time for the earlier solution. Initially I had this by finding all the matches using a regex and then looking at the first and the last. This does NOT work because rusts `regex` will not find overlapping matches. So i had to write it to find the first match in the beginning of the string and the first match in the reverse.

### SIMD 1

This is how you check for SIMD support:

```bash
$ /lib64/ld-linux-x86-64.so.2 --help
...
Subdirectories of glibc-hwcaps directories, in priority order:
x86-64-v4
x86-64-v3 (supported, searched)
x86-64-v2 (supported, searched)
...
````

I wrote a simple implementation that uses SIMD for part 1 using BurntSushi's [memchr](https://docs.rs/memchr/latest/memchr/) library. This is the first SIMD library I came across and I probably am not using it right - this is actually slower than the naive implementation:

```
ITEM TIME(µs)
rust.day01.part1.realinput/part1 72.91µs
rust.day01.part1_simd.realinput/part1_simd 107.15µs
```

Since this library can only search for 3 1-byte long needles at a time, I have to do 6 searches per string. This is why I only wrote part 1 using this lib, not part 2. So to actually use SIMD better I will need to find a different library or dive in and see how to implement this search. The `ripgrep` library might be useful as a reference here?

I added a rustflag to my `cargo` config to be able to use SIMD:

```
[23/12/01 18:52]ankit@devbox:~/code/aoc/2023/rust/day01 (main \*)$ cat ~/.cargo/config
[alias]
rr = "run --release"

[build]
rustflags = ["-C", "target-cpu=native"] #For SIMD support
```
