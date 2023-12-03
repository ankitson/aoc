# Advent of Code 2023

This folder has solutions for Advent of Code 2023 in various languages. Although it has scaffolding for Rust, Python and C++, I will probably only use one language per day atleast initially.

We use `Justfiles` liberally to perform common tasks. You can run commands from a justfile from the folder its in, or from this folder by prefixing the path: i.e `cd rust && just run 5` is equivalent to `just rust/run 5`

## Status

<tstart></tstart>
|ITEM | TIME(ns)| TIME(µs)| TIME(ms)| TIME(s)|
|--------------------------------------------------|---------------|---------------|---------------|---------------|
|rust.day01.part1.realinput/part1 | 72896.11ns| 72.90µs| 0.07ms| 0.00s|
|rust.day01.part2.realinput/part2 | 346756.31ns| 346.76µs| 0.35ms| 0.00s|
|rust.day02.part1.realinput/part1_parsing | 7196.17ns| 7.20µs| 0.01ms| 0.00s|
|rust.day02.part2.realinput/part2_parsing | 8962.86ns| 8.96µs| 0.01ms| 0.00s|
|rust.TOTAL | 435811.45ns| 435.81µs| 0.44ms| 0.00s|
<tend></tend>

## Fetching Puzzles

`just fetch <x>` will download the puzzle text and input for that day. The [aoc-cli](https://github.com/scarvalhojr/aoc-cli/) package must be installed for this to work.

## Templating

`just genday <x>` will generate templates for that day in a given language. These are generated from the `template` dirs using the `genday.sh` script.

## Running

`just run <x>` will run that day's solution

## Benchmarking

`just bench <x>` will benchmark that day's solution.

Python uses `pyperf` to run benchmarks, and Rust uses `criterion`.

After running the benchmarks from the language folder, you can run `just summarize_bench` from this folder to summarize the results from all languages into combined simple `csv` and `txt` files.

## Other

The rust folder has many other actions to e.g watch source files and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
