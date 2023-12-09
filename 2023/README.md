# Advent of Code 2023

This folder has solutions for Advent of Code 2023 in various languages. Although it has scaffolding for Rust, Python and C++, I will probably only use one language per day atleast initially.

We use `Justfiles` liberally to perform common tasks. You can run commands from a justfile from the folder its in, or from this folder by prefixing the path: i.e `cd rust && just run 5` is equivalent to `just rust/run 5`

## Status

<tstart></tstart>
|ITEM                                              |            TIME(ns)|            TIME(µs)|       TIME(ms)|        TIME(s)|
|--------------------------------------------------|--------------------|--------------------|---------------|---------------|
|rust.day01.part1.realinput/part1                  |          72896.11ns|             72.90µs|         0.07ms|          0.00s|
|rust.day01.part2.realinput/part2                  |         346756.31ns|            346.76µs|         0.35ms|          0.00s|
|rust.day02.part1.realinput/part1_parsing          |           7430.86ns|              7.43µs|         0.01ms|          0.00s|
|rust.day02.part2.realinput/part2_parsing          |           9032.34ns|              9.03µs|         0.01ms|          0.00s|
|rust.day03.part1.realinput/part1_fast             |         102641.94ns|            102.64µs|         0.10ms|          0.00s|
|rust.day03.part2.realinput/part2_fast             |         221807.52ns|            221.81µs|         0.22ms|          0.00s|
|python.day03.soln1.realinput/part1                |        4784365.16ns|           4784.37µs|         4.78ms|          0.00s|
|python.day03.soln1.realinput/part2                |        5070635.30ns|           5070.64µs|         5.07ms|          0.01s|
|rust.day04.part1.realinput/part1_bitset           |          60679.28ns|             60.68µs|         0.06ms|          0.00s|
|rust.day04.part2.realinput/part2_streamparse      |          80202.08ns|             80.20µs|         0.08ms|          0.00s|
|python.day04.soln1.realinput/part1                |        1457379.75ns|           1457.38µs|         1.46ms|          0.00s|
|python.day04.soln1.realinput/part2                |        1555943.55ns|           1555.94µs|         1.56ms|          0.00s|
|python.day05.part1.realinput/part1                |       22241554.30ns|          22241.55µs|        22.24ms|          0.02s|
|rust.day05.part2.realinput/part2                  |   129860511656.10ns|      129860511.66µs|    129860.51ms|        129.86s|
|rust.day06.part1.realinput/part1                  |            112.06ns|              0.11µs|         0.00ms|          0.00s|
|rust.day06.part2.realinput/part2                  |            361.12ns|              0.36µs|         0.00ms|          0.00s|
|python.day06.soln1.realinput/part1                |          19273.52ns|             19.27µs|         0.02ms|          0.00s|
|python.day06.soln1.realinput/part2                |           4731.52ns|              4.73µs|         0.00ms|          0.00s|
|rust.day07.part1.realinput/part1                  |        3940912.37ns|           3940.91µs|         3.94ms|          0.00s|
|rust.day07.part2.realinput/part2                  |        4413914.03ns|           4413.91µs|         4.41ms|          0.00s|
|rust.day08.part1.realinput/part1                  |         468802.57ns|            468.80µs|         0.47ms|          0.00s|
|rust.day08.part2.realinput/part2                  |        2380844.85ns|           2380.84µs|         2.38ms|          0.00s|
|python.day08.soln1.realinput/part1                |        2911814.50ns|           2911.81µs|         2.91ms|          0.00s|
|python.day08.soln1.realinput/part2                |       16461741.50ns|          16461.74µs|        16.46ms|          0.02s|
|rust.TOTAL                                        |   129872618049.53ns|      129872618.05µs|    129872.62ms|        129.87s|
|python.TOTAL                                      |       54507439.09ns|          54507.44µs|        54.51ms|          0.05s|
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