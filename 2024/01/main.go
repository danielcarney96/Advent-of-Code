package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	part1()
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var left, right []string

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "   ")

		left = append(left, parts[0])
		right = append(right, parts[1])
	}

	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})

	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})

	sum := 0

	for i := 0; i < len(left); i++ {
		l, _ := strconv.ParseFloat(left[i], 64)
		r, _ := strconv.ParseFloat(right[i], 64)

		sum += int(math.Abs(l - r))
	}

	fmt.Println(sum)
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var left, right []string

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "   ")

		left = append(left, parts[0])
		right = append(right, parts[1])
	}

	sum := 0

	for i := 0; i < len(left); i++ {
		count := 0

		for j := 0; j < len(right); j++ {
			if left[i] == right[j] {
				count++
			}
		}

		val, _ := strconv.Atoi(left[i])
		sum += count * val
	}

	fmt.Println(sum)
}
