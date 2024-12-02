package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"slices"

	"golang.org/x/exp/constraints"
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

func Abs[T constraints.Integer](x T) T {
	if x < 0 {
		return -x
	}
	return x
}

func parse(input []byte) ([]uint, []uint) {
	var l1, l2 []uint
	scanner := bufio.NewScanner(bytes.NewReader(input))
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		var n1, n2 uint
		fmt.Sscanf(line, "%d %d", &n1, &n2)
		l1 = append(l1, n1)
		l2 = append(l2, n2)
	}
	check(scanner.Err())
	return l1, l2
}

func part1(input []byte) uint {
	l1, l2 := parse(input)
	slices.Sort(l1)
	slices.Sort(l2)
	d := uint(0)
	for i := range len(l1) {
		d += uint(Abs(int(l1[i]) - int(l2[i])))
	}
	return d
}

func part2(input []byte) uint {
	l1, l2 := parse(input)
	freqMap := make(map[uint]uint)
	for _, num := range l2 {
		freqMap[num]++
	}
	var d uint
	for _, num := range l1 {
		d += num * freqMap[num]
	}
	return d
}

func main() {
	sample1, err := os.ReadFile("./inputs/sample01.txt")
	check(err)
	inp1, err := os.ReadFile("./inputs/day01.txt")
	check(err)

	ans_sample_p1 := part1(sample1)
	fmt.Printf("part1/sample1 = %d\n", ans_sample_p1)
	assertEqual(ans_sample_p1, 11)

	ans_p1 := part1(inp1)
	fmt.Printf("part1/day01 = %d\n", ans_p1)
	assertEqual(ans_p1, 1151792)

	ans_sample_p2 := part2(sample1)
	fmt.Printf("part2/sample1 = %d\n", ans_sample_p2)
	assertEqual(ans_sample_p2, 31)

	ans_p2 := part2(inp1)
	fmt.Printf("part2/day01 = %d\n", ans_p2)
	assertEqual(ans_p2, 21790168)
}
