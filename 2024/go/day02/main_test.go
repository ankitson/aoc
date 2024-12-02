package main

import (
	_ "embed"
	"testing"
)

//go:generate cp -r ../inputs/day02.txt ./day02.txt
//go:embed day02.txt
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
