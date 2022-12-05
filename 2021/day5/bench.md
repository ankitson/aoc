# Setup for benchmarking

Using dhat (memory allocation) + criterion (cpu time)

Run `bench.sh` on a clean git tree. It aborts if the git tree is not clean - this ensures every benchmark result is tied to a particular set of code.

`bench.sh` will run dhat and criterion and save the results to bench_results/<git_commit_id>

`dhat` runs only when the allocator is added to the code. For now, this script does not automatically add/remove code to switch to the dhat heap.


