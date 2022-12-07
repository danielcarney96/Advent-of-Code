package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	part1()
	part2()
}

func part1() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		for i := range line {
			potentialMatch := line[i : i+4]

			if !hasDuplicates(potentialMatch) {
				fmt.Printf("Part 1: %d\n", 4+i)
				return
			}
		}
	}
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		for i := range line {
			potentialMatch := line[i : i+14]

			if !hasDuplicates(potentialMatch) {
				fmt.Printf("Part 2: %d\n", 14+i)
				return
			}
		}
	}
}

func hasDuplicates(word string) bool {
	for i := 0; i < len(word); i++ {
		for j := i + 1; j < len(word); j++ {
			if word[i] == word[j] {
				return true
			}
		}
	}

	return false
}
