Use the [Justfiles](https://github.com/casey/just#) to run or benchmark packages:

```bash
> just run rust/day05
> just bench rust/day10
> just run python/day07
> just run python/day12
> just summarize_bench
```

The `summarize_bench` script does not run any benchmarks on its own, just pretty prints & writes the combined output of benches for each language.