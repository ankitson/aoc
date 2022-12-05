#!/bin/bash

git diff-index --quiet HEAD --;
GIT_IS_CLEAN=$?;
GIT_COMMIT=$(git rev-parse --short HEAD);

cargo build --release
BUILD_STATUS=$?;
if [ $BUILD_STATUS -ne 0 ]; then
  echo "cannot build project";
  exit 1;
fi;

if [ "$1" = "save" ]; then
  if [ $GIT_IS_CLEAN -ne "0" ]; then
    echo "git staging is dirty";
    exit 1;
  fi;

  echo "Benching $GIT_COMMIT";
  echo "Saving results at bench_results/${GIT_COMMIT}/";
  mkdir -p "bench_results/${GIT_COMMIT}/criterion/";
fi;

echo "Running cargo bench (criterion)...";
cargo -q bench

if [ "$1" = "save" ]; then
  cp -R ../target/criterion/day5*/ "bench_results/${GIT_COMMIT}/criterion/"
fi;

echo "Running release binary (dhat)...";
cargo run --release
if [ "$1" = "save" ]; then
  mv dhat-heap.json "bench_results/${GIT_COMMIT}/dhat-heap.json";

  echo "Results ready at bench_results/${GIT_COMMIT}/";
fi;


#echo "Adding to current commit";
#git add "bench_results/${GIT_COMMIT}";
#git commit --amend --no-edit;

