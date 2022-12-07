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
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	total := 0

	for scanner.Scan() {
		pairs := scanner.Text()

		workers := strings.Split(pairs, ",")

		worker1Times := strings.Split(workers[0], "-")
		worker2Times := strings.Split(workers[1], "-")

		worker1Start, _ := strconv.Atoi(worker1Times[0])
		worker1End, _ := strconv.Atoi(worker1Times[1])
		worker2Start, _ := strconv.Atoi(worker2Times[0])
		worker2End, _ := strconv.Atoi(worker2Times[1])

		if worker1Start >= worker2Start && worker1End <= worker2End {
			total++
		} else if worker2Start >= worker1Start && worker2End <= worker1End {
			total++
		}
	}

	fmt.Printf("Part 1: %d\n", total)
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	total := 0

	for scanner.Scan() {
		pairs := scanner.Text()

		workers := strings.Split(pairs, ",")

		worker1Times := strings.Split(workers[0], "-")
		worker2Times := strings.Split(workers[1], "-")

		worker1Start, _ := strconv.Atoi(worker1Times[0])
		worker1End, _ := strconv.Atoi(worker1Times[1])
		worker2Start, _ := strconv.Atoi(worker2Times[0])
		worker2End, _ := strconv.Atoi(worker2Times[1])

		if worker1Start <= worker2End && worker2Start <= worker1End {
			total++
		}
	}

	fmt.Printf("Part 2: %d\n", total)
}
