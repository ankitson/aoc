#!/bin/bash

cargo bench -q --bench corebench \
  -p day1 \
  -p day2 \
  -p day3 \
  -p day4 \
  -p day5 \
-- --noplot --output-format bencher --color never --sample-size 10 | tee raw_bench.txt
