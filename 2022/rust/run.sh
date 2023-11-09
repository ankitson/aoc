#!/bin/bash

export RUSTFLAGS=-Awarnings

cargo run -q --bin day1; printf "\n";
cargo run -q --bin day2; printf "\n";
cargo run -q --bin day3; printf "\n";
cargo run -q --bin day4; printf "\n";
cargo run -q --bin day5; printf "\n";

unset RUSTFLAGS
