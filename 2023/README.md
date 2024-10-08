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
|rust.day09.part1.realinput/part1_noalloc          |         114911.34ns|            114.91µs|         0.11ms|          0.00s|
|rust.day09.part2.realinput/part2_noalloac         |         117004.17ns|            117.00µs|         0.12ms|          0.00s|
|rust.day11.part1.realinput/part1                  |       36670044.74ns|          36670.04µs|        36.67ms|          0.04s|
|rust.day11.part2.realinput/part2                  |       35967975.32ns|          35967.98µs|        35.97ms|          0.04s|
|python.day13.soln1.realinput/part1                |        7569023.80ns|           7569.02µs|         7.57ms|          0.01s|
|python.day13.soln1.realinput/part2                |      515729019.37ns|         515729.02µs|       515.73ms|          0.52s|
|rust.day14.part1.realinput/part1                  |         150950.06ns|            150.95µs|         0.15ms|          0.00s|
|rust.day14.part2.realinput/part2                  |       91259454.43ns|          91259.45µs|        91.26ms|          0.09s|
|rust.day15.part1.realinput/part1                  |          35355.90ns|             35.36µs|         0.04ms|          0.00s|
|rust.day15.part2.realinput/part2                  |         122060.35ns|            122.06µs|         0.12ms|          0.00s|
|rust.day16.part1.realinput/part1                  |        1362955.17ns|           1362.96µs|         1.36ms|          0.00s|
|rust.day16.part2.realinput/part2                  |      325412274.97ns|         325412.27µs|       325.41ms|          0.33s|
|rust.day19.part1.realinput/part1                  |         684850.08ns|            684.85µs|         0.68ms|          0.00s|
|rust.day19.part2.realinput/part2                  |         533139.11ns|            533.14µs|         0.53ms|          0.00s|
|rust.day22.part1.realinput/part1                  |      200757058.15ns|         200757.06µs|       200.76ms|          0.20s|
|rust.day22.part2.realinput/part2                  |      201071477.60ns|         201071.48µs|       201.07ms|          0.20s|
|rust.TOTAL                                        |   130766877560.93ns|      130766877.56µs|    130766.88ms|        130.77s|
|python.TOTAL                                      |      577805482.26ns|         577805.48µs|       577.81ms|          0.58s|
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
just --list` to view available actions.
iles and rebuild, generate memory profiles, generate flamegraphs, run linters etc. Run `just --list` to view available actions.
