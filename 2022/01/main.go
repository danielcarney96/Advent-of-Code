package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	var totals []int
	sum := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()

		if len(text) > 0 {
			intval, _ := strconv.Atoi(text)
			sum += intval
		} else {
			totals = append(totals, sum)
			sum = 0
		}
	}

	sort.Sort(sort.Reverse(sort.IntSlice(totals)))

	fmt.Printf("Part 1: %d\n", totals[0])
	fmt.Printf("Part 2: %d\n", totals[0]+totals[1]+totals[2])
}
