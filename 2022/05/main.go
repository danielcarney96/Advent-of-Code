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

	stacks := []string{"RNFVLJSM", "PNDZFJWH", "WRCDG", "NBS", "MZWPCBFN", "PRMW", "RTNGLSW", "QTHFNBV", "LMHZNF"}
	var instructions []string

	newLine := false
	for scanner.Scan() {
		line := scanner.Text()

		if len(line) < 1 {
			newLine = true
			continue
		}

		if newLine {
			instruction := strings.ReplaceAll(line, "move ", "")
			instruction = strings.ReplaceAll(instruction, "from ", "")
			instruction = strings.ReplaceAll(instruction, "to ", "")
			instructions = append(instructions, instruction)
		}
	}

	for _, instruction := range instructions {
		steps := strings.Split(instruction, " ")

		move, _ := strconv.Atoi(steps[0])
		from, _ := strconv.Atoi(steps[1])
		to, _ := strconv.Atoi(steps[2])

		plucked := stacks[from-1][len(stacks[from-1])-move:]
		pluckedReverse := reverse(plucked)

		updatedTo := stacks[to-1] + pluckedReverse
		replacedFrom := strings.Replace(reverse(stacks[from-1]), reverse(plucked), "", 1)
		updatedFrom := reverse(replacedFrom)
		stacks[to-1] = updatedTo
		stacks[from-1] = updatedFrom
	}

	var finalString string

	for _, stack := range stacks {
		char := stack[len(stack)-1:]

		finalString += char
	}

	fmt.Printf("Part 1: %s\n", finalString)
}

func part2() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	stacks := []string{"RNFVLJSM", "PNDZFJWH", "WRCDG", "NBS", "MZWPCBFN", "PRMW", "RTNGLSW", "QTHFNBV", "LMHZNF"}
	var instructions []string

	newLine := false
	for scanner.Scan() {
		line := scanner.Text()

		if len(line) < 1 {
			newLine = true
			continue
		}

		if newLine {
			instruction := strings.ReplaceAll(line, "move ", "")
			instruction = strings.ReplaceAll(instruction, "from ", "")
			instruction = strings.ReplaceAll(instruction, "to ", "")
			instructions = append(instructions, instruction)
		}
	}

	for _, instruction := range instructions {
		steps := strings.Split(instruction, " ")

		move, _ := strconv.Atoi(steps[0])
		from, _ := strconv.Atoi(steps[1])
		to, _ := strconv.Atoi(steps[2])

		plucked := stacks[from-1][len(stacks[from-1])-move:]

		updatedTo := stacks[to-1] + plucked
		replacedFrom := strings.Replace(reverse(stacks[from-1]), reverse(plucked), "", 1)
		updatedFrom := reverse(replacedFrom)
		stacks[to-1] = updatedTo
		stacks[from-1] = updatedFrom
	}

	var finalString string

	for _, stack := range stacks {
		char := stack[len(stack)-1:]

		finalString += char
	}

	fmt.Printf("Part 2: %s\n", finalString)
}

func reverse(s string) string {
	rns := []rune(s)

	for i, j := 0, len(rns)-1; i < j; i, j = i+1, j-1 {
		rns[i], rns[j] = rns[j], rns[i]
	}

	return string(rns)
}
