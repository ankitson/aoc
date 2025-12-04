# Advent of Code 2025

This folder has solutions for Advent of Code 2025 in various languages.

We use `Justfiles` liberally to perform common tasks. You can run commands from a justfile from the folder its in, or from this folder by prefixing the path: i.e `cd rust && just run 5` is equivalent to `just rust/run 5`

## Status

<tstart></tstart>
|ITEM                                              |            TIME(ns)|            TIME(µs)|       TIME(ms)|        TIME(s)|
|--------------------------------------------------|--------------------|--------------------|---------------|---------------|
|rust.day01.part1.realinput/part1                  |         204939.25ns|            204.94µs|         0.20ms|          0.00s|
|rust.day01.part2.realinput/part2                  |         205767.64ns|            205.77µs|         0.21ms|          0.00s|
|python.day01.soln1.realinput/part1                |        1874990.75ns|           1874.99µs|         1.87ms|          0.00s|
|python.day01.soln1.realinput/part2                |        2353768.36ns|           2353.77µs|         2.35ms|          0.00s|
|rust.day02.part1.realinput/part1                  |       57998174.02ns|          57998.17µs|        58.00ms|          0.06s|
|rust.day02.part2.realinput/part2                  |      144576348.50ns|         144576.35µs|       144.58ms|          0.14s|
|python.day02.soln1.realinput/part1                |     1253391303.80ns|        1253391.30µs|      1253.39ms|          1.25s|
|python.day02.soln1.realinput/part2                |     5623301734.67ns|        5623301.73µs|      5623.30ms|          5.62s|
|rust.day03.part1.realinput/part1                  |          77999.62ns|             78.00µs|         0.08ms|          0.00s|
|rust.day03.part2.realinput/part2                  |         110906.37ns|            110.91µs|         0.11ms|          0.00s|
|python.day03.soln1.realinput/part1                |        4429397.71ns|           4429.40µs|         4.43ms|          0.00s|
|python.day03.soln1.realinput/part2                |        6330911.06ns|           6330.91µs|         6.33ms|          0.01s|
|rust.day04.part1.realinput/part1                  |        1286821.80ns|           1286.82µs|         1.29ms|          0.00s|
|rust.day04.part2.realinput/part2                  |       19152087.41ns|          19152.09µs|        19.15ms|          0.02s|
|rust.TOTAL                                        |      223613044.61ns|         223613.04µs|       223.61ms|          0.22s|
|python.TOTAL                                      |     6891682106.34ns|        6891682.11µs|      6891.68ms|          6.89s|
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
iles and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
iles and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
iles and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
es and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
iles and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
