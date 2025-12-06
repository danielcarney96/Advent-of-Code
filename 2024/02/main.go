package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Direction string

const (
	INCREASING Direction = "increasing"
	DECREASING Direction = "decreasing"
)

func main() {
	part1()
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	safe_count := 0

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")

		var expected Direction
		safe := true

		first, _ := strconv.Atoi(parts[0])
		second, _ := strconv.Atoi(parts[1])

		// Check increasing or decreasing
		if first > second {
			expected = DECREASING
		} else {
			expected = INCREASING
		}

		for i := 1; i < len(parts); i++ {
			current, _ := strconv.Atoi(parts[i])
			prev, _ := strconv.Atoi(parts[i-1])

			is_safe := isSafe(prev, current, expected)

			if !is_safe {
				safe = false
				break
			}
		}

		if safe {
			safe_count++
		}
	}

	fmt.Println(safe_count)
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	safe_count := 0

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")

		var direction Direction
		safe := true

		first, _ := strconv.Atoi(parts[0])
		second, _ := strconv.Atoi(parts[1])

		// Check increasing or decreasing
		if first > second {
			direction = DECREASING
		} else {
			direction = INCREASING
		}

		for i := 1; i < len(parts); i++ {
			if i == 1 {

			}

			current, _ := strconv.Atoi(parts[i])
			prev, _ := strconv.Atoi(parts[i-1])

			is_safe := isSafe(prev, current, direction)

			if !is_safe {
				is_safe_skipped := false

				if i < len(parts)-1 {
					next, _ := strconv.Atoi(parts[i+1])

					is_safe_skipped = isSafe(prev, next, direction)

					if is_safe_skipped {
						i++
					}
				}

				if !is_safe_skipped {
					safe = false
					break
				}
			}
		}

		if safe {
			safe_count++
		}
	}

	fmt.Println(safe_count)
}

func isSafe(first int, second int, direction Direction) bool {
	if direction == INCREASING && first > second {
		return false
	} else if direction == DECREASING && first < second {
		return false
	}

	if !checkDiffIsWithinRange(first, second) {
		return false
	}

	return true
}

func checkDiffIsWithinRange(first int, second int) bool {
	return math.Abs(float64(first-second)) <= 3 && math.Abs(float64(first-second)) >= 1
}
