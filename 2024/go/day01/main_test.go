package main

import (
	_ "embed"
	"testing"
)

//go:generate cp -r ../inputs/day01.txt ./day01.txt
//go:embed day01.txt
var input []byte

func Benchmark_Parse(b *testing.B) {
	for i := 0; i < b.N; i++ {
		parse(input)
	}
}

func Benchmark_Part1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part1(input)
	}
}

func Benchmark_Part2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part2(input)
	}
}
