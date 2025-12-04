package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"

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

type Input = [][]int
type Output = int

func parse(input []byte) Input {
	var reports [][]int
	scanner := bufio.NewScanner(bytes.NewReader(input))
	for i := 0; scanner.Scan(); i++ {
		line := scanner.Text()
		var levels []int
		var numbers = strings.Split(line, " ")
		for _, numStr := range numbers {
			num, err := strconv.ParseUint(numStr, 10, 64)
			check(err)
			levels = append(levels, int(num))
		}
		reports = append(reports, levels)
	}
	check(scanner.Err())
	return reports
}

func Abs[T constraints.Integer](x T) T {
	if x < 0 {
		return -x
	}
	return x
}

func part1(input []byte) Output {
	reports := parse(input)
	total := 0
	for _, report := range reports {
		incr := report[1] > report[0]
		is_valid := true
		for i := 0; i < len(report)-1; i++ {
			curr, next := report[i], report[i+1]
			abs_diff := Abs(curr - next)
			abs_cond := abs_diff >= 1 && abs_diff <= 3
			order_cond := (incr && next > curr) || (!incr && curr > next)
			is_valid = is_valid && abs_cond && order_cond
		}
		if is_valid {
			total += 1
		}
	}
	return total
}

// [10,1,2,3,4,5,6]
// we need to try excluding each elem once.
// exclude the first element each time

func check_pair(a int, b int, incr bool) bool {
	abs_diff := Abs(a - b)
	abs_cond := abs_diff >= 1 && abs_diff <= 3
	order_cond := (incr && b > a) || (!incr && a > b)
	return abs_cond && order_cond
}

// [1,10,2,3,4,5]
func part2(input []byte) Output {
	reports := parse(input)
	total := 0
	for _, report := range reports {
		incr := report[1] > report[0]
		exclude_idx := -1
		is_valid := true
		skip := true
		for i := 0; i < len(report)-1; i++ {
			is_valid_here := check_pair(report[i], report[i+1], incr)
			if !is_valid_here && exclude_idx == -1 {
				exclude_idx = i
				if i > 0 {
					skip = check_pair(report[i-1], report[i+1], incr)
				}
			}
			is_valid = is_valid && (is_valid_here || exclude_idx == i) && skip
			if !skip {
				skip = true
			}
			fmt.Println("i=", i, " curr=", report[i], " next=", report[i+1], " exlude=", exclude_idx, " valid_here=", is_valid_here, " valid=", is_valid)
		}
		if is_valid {
			fmt.Println(report, "is valid")
			total += 1
		}
	}
	return total
}

func main() {
	sample1, err := os.ReadFile("./inputs/sample02.txt")
	check(err)
	inp1, err := os.ReadFile("./inputs/day02.txt")
	check(err)

	fmt.Println("Welcome to Day 02!!")
	ans_sample_p1 := part1(sample1)
	fmt.Printf("part1/sample1 = %d\n", ans_sample_p1)
	assertEqual(ans_sample_p1, 2)

	ans_p1 := part1(inp1)
	fmt.Printf("part1/day02 = %d\n", ans_p1)
	assertEqual(ans_p1, 598)

	ans_sample_p2 := part2(sample1)
	fmt.Printf("part2/sample1 = %d\n", ans_sample_p2)
	assertEqual(ans_sample_p2, 4)

	ans_p2 := part2(inp1)
	fmt.Printf("part2/day02 = %d\n", ans_p2)
	assertEqual(ans_p2, 634)
}
