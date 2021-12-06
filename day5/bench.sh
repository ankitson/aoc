#!/bin/bash

$(git diff-index --quiet HEAD --);
GIT_IS_CLEAN=$?;
GIT_COMMIT=$(git rev-parse --short HEAD);

if [ $GIT_IS_CLEAN -ne 0 ]; then
  echo "git staging is dirty";
  exit 1;
fi;

echo "Benching $GIT_COMMIT";

mkdir -p "bench_results/${GIT_COMMIT}/criterion/";

echo "Running cargo bench (criterion)...";
cargo bench
cp -R ../target/criterion/day5-part1/ "bench_results/${GIT_COMMIT}/criterion/"


echo "Running release binary (dhat)...";
cargo run --release
mv dhat-heap.json "bench_results/${GIT_COMMIT}/dhat-heap.json"

echo "Results ready at bench_results/${GIT_COMMIT}/"
echo `ls -l bench_results/${GIT_COMMIT}/"
