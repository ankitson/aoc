# Advent of Code 2024

This folder has solutions for Advent of Code 2024 in various languages.

We use `Justfiles` liberally to perform common tasks. You can run commands from a justfile from the folder its in, or from this folder by prefixing the path: i.e `cd rust && just run 5` is equivalent to `just rust/run 5`

## Status

<tstart></tstart>
|ITEM                                              |            TIME(ns)|            TIME(µs)|       TIME(ms)|        TIME(s)|
|--------------------------------------------------|--------------------|--------------------|---------------|---------------|
|rust.day01.part1.realinput/part1                  |          59995.45ns|             60.00µs|         0.06ms|          0.00s|
|rust.day01.part1.realinput/part1_par              |         381488.07ns|            381.49µs|         0.38ms|          0.00s|
|rust.day01.part2.realinput/part2                  |          79926.75ns|             79.93µs|         0.08ms|          0.00s|
|rust.day01.part2.realinput/part2_par              |         432437.68ns|            432.44µs|         0.43ms|          0.00s|
|rust.day02.part1.realinput/part1                  |         120680.93ns|            120.68µs|         0.12ms|          0.00s|
|rust.day02.part2.realinput/part2                  |         121848.11ns|            121.85µs|         0.12ms|          0.00s|
|rust.day03.part1.realinput/part1                  |         229500.10ns|            229.50µs|         0.23ms|          0.00s|
|rust.day03.part2.realinput/part2                  |         330059.67ns|            330.06µs|         0.33ms|          0.00s|
|rust.TOTAL                                        |        1755936.77ns|           1755.94µs|         1.76ms|          0.00s|
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
