#!/bin/bash

cargo bench -q --bench corebench -- --noplot --output-format bencher --color never --sample-size 10 | tee raw_bench.txt
