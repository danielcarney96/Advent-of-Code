package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var queuedAdditions []int

	for scanner.Scan() {
		line := scanner.Text()

		parts := strings.Split(line, " ")

		if parts[0] == "noop" {
			queuedAdditions = append(queuedAdditions, 0)
		} else {
			val, _ := strconv.Atoi(parts[1])
			queuedAdditions = append(queuedAdditions, 0)
			queuedAdditions = append(queuedAdditions, val)
		}
	}

	xRegister := 1
	sum := 0

	for i, addition := range queuedAdditions {
		if i == 19 || i == 59 || i == 99 || i == 139 || i == 179 || i == 219 {
			sum += (i + 1) * xRegister
		}

		xRegister += addition
	}

	fmt.Printf("Part 1: %d\n", sum)
}
