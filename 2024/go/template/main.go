package main

import (
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func assertEqual[T comparable](got, want T) {
	if got != want {
		fmt.Printf("Error: got %v, want %v\n", got, want)
	}
}

type Input = string
type Output = string

func parse(input []byte) Input {
	panic("unimplemented")
}

func part1(input []byte) Output {
	panic("unimplemented")
}

func part2(input []byte) Output {
	panic("unimplemented")
}

func main() {
	sample1, err := os.ReadFile("./inputs/sample{DAY_NUM}.txt")
	check(err)
	inp1, err := os.ReadFile("./inputs/day{DAY_NUM}.txt")
	check(err)

	fmt.Println("Welcome to Day {DAY_NUM}!!")
	ans_sample_p1 := part1(sample1)
	fmt.Printf("part1/sample1 = %d\n", ans_sample_p1)
	assertEqual(ans_sample_p1, "")

	ans_p1 := part1(inp1)
	fmt.Printf("part1/day{DAY_NUM} = %d\n", ans_p1)
	assertEqual(ans_p1, "")

	ans_sample_p2 := part2(sample1)
	fmt.Printf("part2/sample1 = %d\n", ans_sample_p2)
	assertEqual(ans_sample_p2, "")

	ans_p2 := part2(inp1)
	fmt.Printf("part2/day{DAY_NUM} = %d\n", ans_p2)
	assertEqual(ans_p2, "")
}
