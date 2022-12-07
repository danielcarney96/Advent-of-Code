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

	total := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		lineFirstHalf := removeDuplicateCharsFromString(line[0 : len(line)/2])
		lineSecondHalf := removeDuplicateCharsFromString(line[len(line)/2:])

		for _, firstHalfChar := range lineFirstHalf {
			for _, secondHalfChar := range lineSecondHalf {
				if firstHalfChar == secondHalfChar {
					position := positionInAlpha(firstHalfChar)

					total += position
				}
			}
		}
	}

	fmt.Printf("Part 1: %d\n", total)
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()

	total := 0
	var lines []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		lines = append(lines, removeDuplicateCharsFromString(line))

		if len(lines) > 2 {

			var matching string
			var final rune

			for _, lineChar1 := range lines[0] {
				for _, lineChar2 := range lines[1] {
					if lineChar1 == lineChar2 {
						matching += string(lineChar1)
					}
				}
			}

		out:
			for _, lineChar3 := range lines[2] {
				for _, matchingChar := range matching {
					if lineChar3 == matchingChar {
						final = lineChar3
						break out
					}
				}
			}

			total += positionInAlpha(final)
			lines = nil
		}
	}

	fmt.Printf("Part 2: %d\n", total)
}

func removeDuplicateCharsFromString(word string) string {
	var unique string

	for _, char := range word {
		found := false

		for _, uniqueChar := range unique {
			if char == uniqueChar {
				found = true
				break
			}
		}

		if !found {
			unique += string(char)
		}
	}

	return unique
}

func positionInAlpha(letter rune) int {
	alphabet := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

	for i, alpha := range alphabet {
		if letter == alpha {
			return i + 1
		}
	}

	return 0
}
